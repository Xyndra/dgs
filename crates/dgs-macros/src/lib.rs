//! Proc-macro DSL for the [`dgs`] dynamic geometry crate.
//!
//! This crate is an implementation detail; use the umbrella [`dgs`] crate,
//! which re-exports the macro so `dgs` is the only dependency you need:
//!
//! ```
//! use dgs::dgs;
//!
//! let pts = vec![(1.0, 1.0), (2.0, 3.0)];
//! let scene = dgs! {
//!     viewport [-5, -5] to [5, 5] size (400, 400);
//!     theme light;
//!     grid spacing 1;
//!     axes;
//!     point A (0, 0);
//!     // Rust control flow over outside data:
//!     for (x, y) in pts {
//!         point (x, y) color blue;
//!     }
//!     if true {
//!         line A -> (2, 3);
//!     }
//!     // Raw Rust escape hatch (operates on `__scene`):
//!     #__scene = __scene.grid(false);
//!     eq "sin(x) / x" x in [-5, 5] color "#ff6600";
//! };
//! assert!(scene.to_svg().starts_with("<svg"));
//! ```
//!
//! [`dgs`]: https://docs.rs/dgs

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{braced, parse_macro_input, Expr, Ident, LitStr, Pat, Token};

mod kw {
    syn::custom_keyword!(viewport);
    syn::custom_keyword!(theme);
    syn::custom_keyword!(grid);
    syn::custom_keyword!(axes);
    syn::custom_keyword!(point);
    syn::custom_keyword!(line);
    syn::custom_keyword!(circle);
    syn::custom_keyword!(ellipse);
    syn::custom_keyword!(arc);
    syn::custom_keyword!(semicircle);
    syn::custom_keyword!(polygon);
    syn::custom_keyword!(curve);
    syn::custom_keyword!(curve_param);
    syn::custom_keyword!(eq);
    syn::custom_keyword!(eq_param);
    syn::custom_keyword!(to);
    syn::custom_keyword!(size);
    syn::custom_keyword!(light);
    syn::custom_keyword!(dark);
    syn::custom_keyword!(spacing);
    syn::custom_keyword!(width);
    syn::custom_keyword!(color);
    syn::custom_keyword!(stroke);
    syn::custom_keyword!(fill);
    syn::custom_keyword!(radius);
    syn::custom_keyword!(rx);
    syn::custom_keyword!(ry);
    syn::custom_keyword!(rot);
    syn::custom_keyword!(from);
    syn::custom_keyword!(cw);
    syn::custom_keyword!(ccw);
    syn::custom_keyword!(center);
    syn::custom_keyword!(label_size);
}

struct DgsScene {
    stmts: Vec<Stmt>,
}

enum Stmt {
    Viewport {
        x1: Expr,
        y1: Expr,
        x2: Expr,
        y2: Expr,
        width: Expr,
        height: Expr,
    },
    Theme(ThemeKind),
    Grid {
        spacing: Option<Expr>,
        width: Option<Expr>,
        color: Option<ColorSpec>,
    },
    Axes {
        width: Option<Expr>,
        color: Option<ColorSpec>,
        label_size: Option<Expr>,
    },
    Push {
        kind: ObjectKind,
        color: Option<ColorSpec>,
        stroke: Option<Expr>,
        fill: Option<ColorSpec>,
    },
    For {
        pat: Pat,
        iter: Expr,
        body: Vec<Stmt>,
    },
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    Loop {
        body: Vec<Stmt>,
    },
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_branch: Option<ElseBranch>,
    },
    /// Raw Rust statement (operates on `__scene`).
    Raw(syn::Stmt),
}

enum ElseBranch {
    Else(Vec<Stmt>),
    ElseIf(Box<Stmt>),
}

enum ThemeKind {
    Light,
    Dark,
}

enum ColorSpec {
    Named(Ident),
    Hex(LitStr),
}

impl ColorSpec {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            // Panics on invalid names at runtime, like Typst does.
            ColorSpec::Named(ident) => {
                let name = ident.to_string();
                quote! { ::dgs::__private::color(#name) }
            }
            ColorSpec::Hex(lit) => {
                let s = lit.value();
                quote! { ::dgs::__private::color(#s) }
            }
        }
    }
}

/// A point reference: either an identifier (`A`) referring to a named point,
/// or a coordinate tuple (`(3, 4)`).
enum PointRefSpec {
    Named(Ident),
    Coords(Expr, Expr),
}

impl PointRefSpec {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            PointRefSpec::Named(ident) => {
                let name = ident.to_string();
                quote! { ::dgs::__private::PointRef::named(#name) }
            }
            PointRefSpec::Coords(x, y) => {
                quote! { ::dgs::__private::PointRef::at(#x as f64, #y as f64) }
            }
        }
    }

    fn span(&self) -> proc_macro2::Span {
        match self {
            PointRefSpec::Named(ident) => ident.span(),
            PointRefSpec::Coords(x, _) => x.span(),
        }
    }
}

enum ObjectKind {
    PointNamed {
        name: Ident,
        x: Expr,
        y: Expr,
        size: Option<Expr>,
    },
    PointAt {
        x: Expr,
        y: Expr,
        size: Option<Expr>,
    },
    Line {
        from: PointRefSpec,
        to: PointRefSpec,
    },
    Circle {
        center: PointRefSpec,
        radius: Expr,
    },
    Ellipse {
        center: PointRefSpec,
        rx: Expr,
        ry: Expr,
        rotation: Option<Expr>,
    },
    Arc {
        center: PointRefSpec,
        radius: Expr,
        start_angle: Expr,
        end_angle: Expr,
    },
    Semicircle {
        from: PointRefSpec,
        to: PointRefSpec,
        center: Option<PointRefSpec>,
        dir: SemicircleDirKind,
    },
    Polygon {
        points: Vec<PointRefSpec>,
    },
    Curve {
        expr: LitStr,
        var: Ident,
        t_min: Expr,
        t_max: Expr,
    },
    CurveParam {
        x_expr: LitStr,
        y_expr: LitStr,
        t_min: Expr,
        t_max: Expr,
    },
}

enum SemicircleDirKind {
    Cw,
    Ccw,
}

impl ToTokens for SemicircleDirKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = match self {
            SemicircleDirKind::Cw => format_ident!("Clockwise"),
            SemicircleDirKind::Ccw => format_ident!("CounterClockwise"),
        };
        tokens.extend(quote! { ::dgs::__private::SemicircleDir::#ident });
    }
}

fn parse_stmt_list(input: ParseStream) -> syn::Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while !input.is_empty() {
        stmts.push(input.parse::<Stmt>()?);
        // `;` separators are optional (one per line is the norm).
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
        }
    }
    Ok(stmts)
}

fn parse_braced_body(input: ParseStream) -> syn::Result<Vec<Stmt>> {
    let content;
    braced!(content in input);
    parse_stmt_list(&content)
}

impl Parse for DgsScene {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DgsScene {
            stmts: parse_stmt_list(input)?,
        })
    }
}

impl Parse for Stmt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Rust control flow and escape hatch first.
        if input.peek(Token![for]) {
            input.parse::<Token![for]>()?;
            let pat: Pat = Pat::parse_single(input)?;
            input.parse::<Token![in]>()?;
            let iter: Expr = Expr::parse_without_eager_brace(input)?;
            let body = parse_braced_body(input)?;
            return Ok(Stmt::For { pat, iter, body });
        }
        if input.peek(Token![while]) {
            input.parse::<Token![while]>()?;
            let cond: Expr = Expr::parse_without_eager_brace(input)?;
            let body = parse_braced_body(input)?;
            return Ok(Stmt::While { cond, body });
        }
        if input.peek(Token![loop]) {
            input.parse::<Token![loop]>()?;
            let body = parse_braced_body(input)?;
            return Ok(Stmt::Loop { body });
        }
        if input.peek(Token![if]) {
            return parse_if(input);
        }
        if input.peek(Token![#]) {
            input.parse::<Token![#]>()?;
            return Ok(Stmt::Raw(input.parse::<syn::Stmt>()?));
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(kw::viewport) {
            input.parse::<kw::viewport>()?;
            let (x1, y1) = parse_bracket_pair(input)?;
            let (x2, y2) = if input.peek(kw::to) {
                input.parse::<kw::to>()?;
                parse_bracket_pair(input)?
            } else {
                (syn::parse_quote!(10.0), syn::parse_quote!(10.0))
            };
            input.parse::<kw::size>()?;
            let (width, height) = parse_paren_pair(input)?;
            Ok(Stmt::Viewport {
                x1,
                y1,
                x2,
                y2,
                width,
                height,
            })
        } else if lookahead.peek(kw::theme) {
            input.parse::<kw::theme>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::light) {
                input.parse::<kw::light>()?;
                Ok(Stmt::Theme(ThemeKind::Light))
            } else if lookahead.peek(kw::dark) {
                input.parse::<kw::dark>()?;
                Ok(Stmt::Theme(ThemeKind::Dark))
            } else {
                Err(lookahead.error())
            }
        } else if lookahead.peek(kw::grid) {
            input.parse::<kw::grid>()?;
            let mut spacing = None;
            let mut width = None;
            let mut color = None;
            loop {
                let lookahead = input.lookahead1();
                if lookahead.peek(kw::spacing) {
                    input.parse::<kw::spacing>()?;
                    spacing = Some(input.parse::<Expr>()?);
                } else if lookahead.peek(kw::width) {
                    input.parse::<kw::width>()?;
                    width = Some(input.parse::<Expr>()?);
                } else if lookahead.peek(kw::color) {
                    input.parse::<kw::color>()?;
                    color = Some(input.parse::<ColorSpec>()?);
                } else {
                    break;
                }
            }
            Ok(Stmt::Grid {
                spacing,
                width,
                color,
            })
        } else if lookahead.peek(kw::axes) {
            input.parse::<kw::axes>()?;
            let mut width = None;
            let mut color = None;
            let mut label_size = None;
            loop {
                let lookahead = input.lookahead1();
                if lookahead.peek(kw::width) {
                    input.parse::<kw::width>()?;
                    width = Some(input.parse::<Expr>()?);
                } else if lookahead.peek(kw::color) {
                    input.parse::<kw::color>()?;
                    color = Some(input.parse::<ColorSpec>()?);
                } else if lookahead.peek(kw::label_size) {
                    input.parse::<kw::label_size>()?;
                    label_size = Some(input.parse::<Expr>()?);
                } else {
                    break;
                }
            }
            Ok(Stmt::Axes {
                width,
                color,
                label_size,
            })
        } else if lookahead.peek(kw::point)
            || lookahead.peek(kw::line)
            || lookahead.peek(kw::circle)
            || lookahead.peek(kw::ellipse)
            || lookahead.peek(kw::arc)
            || lookahead.peek(kw::semicircle)
            || lookahead.peek(kw::polygon)
            || lookahead.peek(kw::curve)
            || lookahead.peek(kw::curve_param)
            || lookahead.peek(kw::eq)
            || lookahead.peek(kw::eq_param)
        {
            let kind = parse_object(input)?;
            let mut color = None;
            let mut stroke = None;
            let mut fill = None;
            loop {
                let lookahead = input.lookahead1();
                if lookahead.peek(kw::color) {
                    input.parse::<kw::color>()?;
                    color = Some(input.parse::<ColorSpec>()?);
                } else if lookahead.peek(kw::stroke) {
                    input.parse::<kw::stroke>()?;
                    stroke = Some(input.parse::<Expr>()?);
                } else if lookahead.peek(kw::fill) {
                    input.parse::<kw::fill>()?;
                    fill = Some(input.parse::<ColorSpec>()?);
                } else {
                    break;
                }
            }
            Ok(Stmt::Push {
                kind,
                color,
                stroke,
                fill,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

fn parse_if(input: ParseStream) -> syn::Result<Stmt> {
    input.parse::<Token![if]>()?;
    let cond: Expr = Expr::parse_without_eager_brace(input)?;
    let then_body = parse_braced_body(input)?;
    let else_branch = if input.peek(Token![else]) {
        input.parse::<Token![else]>()?;
        if input.peek(Token![if]) {
            Some(ElseBranch::ElseIf(Box::new(parse_if(input)?)))
        } else {
            Some(ElseBranch::Else(parse_braced_body(input)?))
        }
    } else {
        None
    };
    Ok(Stmt::If {
        cond,
        then_body,
        else_branch,
    })
}

impl Parse for ColorSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(ColorSpec::Named(input.parse::<Ident>()?))
        } else if lookahead.peek(LitStr) {
            Ok(ColorSpec::Hex(input.parse::<LitStr>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for PointRefSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let x: Expr = content.parse()?;
            content.parse::<Token![,]>()?;
            let y: Expr = content.parse()?;
            Ok(PointRefSpec::Coords(x, y))
        } else {
            Ok(PointRefSpec::Named(input.parse::<Ident>()?))
        }
    }
}

fn parse_bracket_pair(input: ParseStream) -> syn::Result<(Expr, Expr)> {
    let content;
    syn::bracketed!(content in input);
    let a: Expr = content.parse()?;
    content.parse::<Token![,]>()?;
    let b: Expr = content.parse()?;
    Ok((a, b))
}

fn parse_paren_pair(input: ParseStream) -> syn::Result<(Expr, Expr)> {
    let content;
    syn::parenthesized!(content in input);
    let a: Expr = content.parse()?;
    content.parse::<Token![,]>()?;
    let b: Expr = content.parse()?;
    Ok((a, b))
}

fn parse_object(input: ParseStream) -> syn::Result<ObjectKind> {
    let lookahead = input.lookahead1();
    if lookahead.peek(kw::point) {
        input.parse::<kw::point>()?;
        // `point A (x, y)` (named) or `point (x, y)` (unnamed)
        let named = input.peek(Ident) && !input.peek(syn::token::Paren);
        let name = if named {
            Some(input.parse::<Ident>()?)
        } else {
            None
        };
        let (x, y) = parse_paren_pair(input)?;
        let size = if input.peek(kw::size) {
            input.parse::<kw::size>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };
        match name {
            Some(name) => Ok(ObjectKind::PointNamed { name, x, y, size }),
            None => Ok(ObjectKind::PointAt { x, y, size }),
        }
    } else if lookahead.peek(kw::line) {
        input.parse::<kw::line>()?;
        let from: PointRefSpec = input.parse()?;
        input.parse::<Token![->]>()?;
        let to: PointRefSpec = input.parse()?;
        Ok(ObjectKind::Line { from, to })
    } else if lookahead.peek(kw::circle) {
        input.parse::<kw::circle>()?;
        let center: PointRefSpec = input.parse()?;
        input.parse::<kw::radius>()?;
        let radius: Expr = input.parse()?;
        Ok(ObjectKind::Circle { center, radius })
    } else if lookahead.peek(kw::ellipse) {
        input.parse::<kw::ellipse>()?;
        let center: PointRefSpec = input.parse()?;
        input.parse::<kw::rx>()?;
        let rx: Expr = input.parse()?;
        input.parse::<kw::ry>()?;
        let ry: Expr = input.parse()?;
        let rotation = if input.peek(kw::rot) {
            input.parse::<kw::rot>()?;
            Some(input.parse::<Expr>()?)
        } else {
            None
        };
        Ok(ObjectKind::Ellipse {
            center,
            rx,
            ry,
            rotation,
        })
    } else if lookahead.peek(kw::arc) {
        input.parse::<kw::arc>()?;
        let center: PointRefSpec = input.parse()?;
        input.parse::<kw::radius>()?;
        let radius: Expr = input.parse()?;
        input.parse::<kw::from>()?;
        let start_angle: Expr = input.parse()?;
        input.parse::<kw::to>()?;
        let end_angle: Expr = input.parse()?;
        Ok(ObjectKind::Arc {
            center,
            radius,
            start_angle,
            end_angle,
        })
    } else if lookahead.peek(kw::semicircle) {
        input.parse::<kw::semicircle>()?;
        let from: PointRefSpec = input.parse()?;
        input.parse::<Token![->]>()?;
        let to: PointRefSpec = input.parse()?;
        let mut dir = SemicircleDirKind::Ccw;
        let mut center = None;
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::cw) {
                input.parse::<kw::cw>()?;
                dir = SemicircleDirKind::Cw;
            } else if lookahead.peek(kw::ccw) {
                input.parse::<kw::ccw>()?;
                dir = SemicircleDirKind::Ccw;
            } else if lookahead.peek(kw::center) {
                input.parse::<kw::center>()?;
                center = Some(input.parse::<PointRefSpec>()?);
            } else {
                break;
            }
        }
        Ok(ObjectKind::Semicircle {
            from,
            to,
            center,
            dir,
        })
    } else if lookahead.peek(kw::polygon) {
        input.parse::<kw::polygon>()?;
        let mut points = vec![input.parse::<PointRefSpec>()?];
        while (input.peek(Ident) || input.peek(syn::token::Paren))
            && !input.peek(kw::color)
            && !input.peek(kw::stroke)
            && !input.peek(kw::fill)
        {
            points.push(input.parse::<PointRefSpec>()?);
        }
        if points.len() < 3 {
            return Err(syn::Error::new(
                points[0].span(),
                "polygon needs at least 3 points",
            ));
        }
        Ok(ObjectKind::Polygon { points })
    } else if lookahead.peek(kw::curve)
        || lookahead.peek(kw::curve_param)
        || lookahead.peek(kw::eq)
        || lookahead.peek(kw::eq_param)
    {
        let is_param = input.peek(kw::curve_param) || input.peek(kw::eq_param);
        if input.peek(kw::curve) {
            input.parse::<kw::curve>()?;
        } else if input.peek(kw::curve_param) {
            input.parse::<kw::curve_param>()?;
        } else if input.peek(kw::eq) {
            input.parse::<kw::eq>()?;
        } else {
            input.parse::<kw::eq_param>()?;
        }
        if is_param {
            let x_expr: LitStr = input.parse()?;
            let y_expr: LitStr = input.parse()?;
            input.parse::<Token![in]>()?;
            let (t_min, t_max) = parse_bracket_pair(input)?;
            Ok(ObjectKind::CurveParam {
                x_expr,
                y_expr,
                t_min,
                t_max,
            })
        } else {
            let expr: LitStr = input.parse()?;
            // `var` defaults to `x`, like Typst's `dgs-eq(expr, var: "x")`.
            let var: Ident = if input.peek(Token![in]) {
                Ident::new("x", expr.span())
            } else {
                input.parse()?
            };
            input.parse::<Token![in]>()?;
            let (t_min, t_max) = parse_bracket_pair(input)?;
            Ok(ObjectKind::Curve {
                expr,
                var,
                t_min,
                t_max,
            })
        }
    } else {
        Err(lookahead.error())
    }
}

impl DgsScene {
    fn expand(self) -> TokenStream {
        let mut body = proc_macro2::TokenStream::new();
        for stmt in self.stmts {
            body.extend(stmt.expand());
        }
        let expanded = quote! {
            {
                #[allow(unused_mut)]
                let mut __scene = ::dgs::__private::Scene::new(
                    ::dgs::__private::Viewport::new(0.0, 0.0, 10.0, 10.0, 400.0, 400.0),
                );
                #body
                __scene
            }
        };
        expanded.into()
    }
}

impl Stmt {
    fn expand(self) -> proc_macro2::TokenStream {
        match self {
            Stmt::Viewport {
                x1,
                y1,
                x2,
                y2,
                width,
                height,
            } => quote! {
                __scene = ::dgs::__private::Scene::viewport(
                    __scene,
                    ::dgs::__private::Viewport::new(#x1 as f64, #y1 as f64, #x2 as f64, #y2 as f64, #width as f64, #height as f64),
                );
            },
            Stmt::Theme(kind) => {
                let theme = match kind {
                    ThemeKind::Light => quote! { ::dgs::__private::Theme::light() },
                    ThemeKind::Dark => quote! { ::dgs::__private::Theme::dark() },
                };
                quote! {
                    __scene = ::dgs::__private::Scene::theme(__scene, #theme);
                }
            }
            Stmt::Grid {
                spacing,
                width,
                color,
            } => {
                let mut out = quote! {
                    __scene = ::dgs::__private::Scene::grid(__scene, true);
                };
                if let Some(spacing) = spacing {
                    out.extend(quote! {
                        __scene = ::dgs::__private::Scene::grid_spacing(__scene, #spacing as f64);
                    });
                }
                if let Some(width) = width {
                    out.extend(quote! {
                        __scene = ::dgs::__private::Scene::grid_width(__scene, #width as f64);
                    });
                }
                if let Some(color) = color {
                    let t = color.to_tokens();
                    out.extend(quote! {
                        __scene = ::dgs::__private::Scene::grid_color(__scene, #t);
                    });
                }
                out
            }
            Stmt::Axes {
                width,
                color,
                label_size,
            } => {
                let mut out = quote! {
                    __scene = ::dgs::__private::Scene::axes(__scene, true);
                };
                if let Some(width) = width {
                    out.extend(quote! {
                        __scene = ::dgs::__private::Scene::axis_width(__scene, #width as f64);
                    });
                }
                if let Some(color) = color {
                    let t = color.to_tokens();
                    out.extend(quote! {
                        __scene = ::dgs::__private::Scene::axis_color(__scene, #t);
                    });
                }
                if let Some(label_size) = label_size {
                    out.extend(quote! {
                        __scene =
                            ::dgs::__private::Scene::axis_label_size(__scene, #label_size as f64);
                    });
                }
                out
            }
            Stmt::Push {
                kind,
                color,
                stroke,
                fill,
            } => {
                let mut obj = kind.expand();
                if let Some(color) = color {
                    let t = color.to_tokens();
                    obj.extend(quote! { .with_color(#t) });
                }
                if let Some(stroke) = stroke {
                    obj.extend(quote! { .with_stroke(#stroke as f64) });
                }
                if let Some(fill) = fill {
                    let t = fill.to_tokens();
                    obj.extend(quote! { .with_fill(#t) });
                }
                quote! {
                    __scene = ::dgs::__private::Scene::push(__scene, #obj);
                }
            }
            Stmt::For { pat, iter, body } => {
                let body = expand_body(body);
                quote! {
                    for #pat in #iter {
                        #body
                    }
                }
            }
            Stmt::While { cond, body } => {
                let body = expand_body(body);
                quote! {
                    while #cond {
                        #body
                    }
                }
            }
            Stmt::Loop { body } => {
                let body = expand_body(body);
                quote! {
                    loop {
                        #body
                    }
                }
            }
            Stmt::If {
                cond,
                then_body,
                else_branch,
            } => {
                let then_body = expand_body(then_body);
                let else_tokens = else_branch.map(|e| e.expand());
                quote! {
                    if #cond {
                        #then_body
                    } #else_tokens
                }
            }
            Stmt::Raw(stmt) => quote! { #stmt },
        }
    }
}

fn expand_body(body: Vec<Stmt>) -> proc_macro2::TokenStream {
    let mut out = proc_macro2::TokenStream::new();
    for stmt in body {
        out.extend(stmt.expand());
    }
    out
}

impl ElseBranch {
    fn expand(self) -> proc_macro2::TokenStream {
        match self {
            ElseBranch::Else(body) => {
                let body = expand_body(body);
                quote! { else { #body } }
            }
            ElseBranch::ElseIf(stmt) => {
                // `stmt` is always Stmt::If; emit `else <if …>` without braces.
                let Stmt::If {
                    cond,
                    then_body,
                    else_branch,
                } = *stmt
                else {
                    unreachable!("else-if branch is always an if statement");
                };
                let then_body = expand_body(then_body);
                let else_tokens = else_branch.map(|e| e.expand());
                quote! { else if #cond { #then_body } #else_tokens }
            }
        }
    }
}

impl ObjectKind {
    /// Expand to an `Object` value expression (before style chaining).
    fn expand(self) -> proc_macro2::TokenStream {
        match self {
            ObjectKind::PointNamed { name, x, y, size } => {
                let name = name.to_string();
                let mut out = quote! {
                    ::dgs::__private::point(#name, #x as f64, #y as f64)
                };
                if let Some(size) = size {
                    out.extend(quote! { .with_size(#size as f64) });
                }
                out
            }
            ObjectKind::PointAt { x, y, size } => {
                let mut out = quote! {
                    ::dgs::__private::point_at(#x as f64, #y as f64)
                };
                if let Some(size) = size {
                    out.extend(quote! { .with_size(#size as f64) });
                }
                out
            }
            ObjectKind::Line { from, to } => {
                let f = from.to_tokens();
                let t = to.to_tokens();
                quote! { ::dgs::__private::line(#f, #t) }
            }
            ObjectKind::Circle { center, radius } => {
                let c = center.to_tokens();
                quote! { ::dgs::__private::circle(#c, #radius as f64) }
            }
            ObjectKind::Ellipse {
                center,
                rx,
                ry,
                rotation,
            } => {
                let c = center.to_tokens();
                let mut out = quote! {
                    ::dgs::__private::ellipse(#c, #rx as f64, #ry as f64)
                };
                if let Some(rotation) = rotation {
                    out.extend(quote! { .with_rotation(#rotation as f64) });
                }
                out
            }
            ObjectKind::Arc {
                center,
                radius,
                start_angle,
                end_angle,
            } => {
                let c = center.to_tokens();
                quote! {
                    ::dgs::__private::arc(#c, #radius as f64, #start_angle as f64, #end_angle as f64)
                }
            }
            ObjectKind::Semicircle {
                from,
                to,
                center,
                dir,
            } => {
                let f = from.to_tokens();
                let t = to.to_tokens();
                let mut out = quote! { ::dgs::__private::semicircle(#f, #t) };
                out.extend(quote! { .with_dir(#dir) });
                if let Some(center) = center {
                    let c = center.to_tokens();
                    out.extend(quote! { .with_center(#c) });
                }
                out
            }
            ObjectKind::Polygon { points } => {
                let pts = points.iter().map(|p| p.to_tokens());
                quote! { ::dgs::__private::polygon(vec![#(#pts),*]) }
            }
            ObjectKind::Curve {
                expr,
                var,
                t_min,
                t_max,
            } => {
                let var = var.to_string();
                quote! {
                    ::dgs::__private::eq(#expr).with_var(#var).with_range(#t_min as f64, #t_max as f64)
                }
            }
            ObjectKind::CurveParam {
                x_expr,
                y_expr,
                t_min,
                t_max,
            } => {
                quote! {
                    ::dgs::__private::eq_param(#x_expr, #y_expr).with_range(#t_min as f64, #t_max as f64)
                }
            }
        }
    }
}

#[proc_macro]
pub fn dgs(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DgsScene);
    parsed.expand()
}
