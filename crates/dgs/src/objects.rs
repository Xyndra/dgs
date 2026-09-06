use crate::color::Color;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

/// Accepts `samples` as int or float (Typst numbers may arrive as floats
/// via CBOR) or null/missing.
fn de_opt_usize<'de, D>(d: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    struct OptUsize;
    impl<'de> serde::de::Visitor<'de> for OptUsize {
        type Value = Option<usize>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an optional unsigned integer")
        }
        fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
        fn visit_some<D2: Deserializer<'de>>(
            self,
            d: D2,
        ) -> Result<Self::Value, D2::Error> {
            d.deserialize_any(Inner)
        }
        fn visit_bool<E: serde::de::Error>(self, _v: bool) -> Result<Self::Value, E> {
            Ok(None)
        }
    }
    struct Inner;
    impl<'de> serde::de::Visitor<'de> for Inner {
        type Value = Option<usize>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an unsigned integer")
        }
        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(Some(v as usize))
        }
        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(Some(v.max(0) as usize))
        }
        fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(Some(v.round().max(0.0) as usize))
        }
        fn visit_u8<E: serde::de::Error>(self, v: u8) -> Result<Self::Value, E> {
            Ok(Some(v as usize))
        }
        fn visit_u16<E: serde::de::Error>(self, v: u16) -> Result<Self::Value, E> {
            Ok(Some(v as usize))
        }
        fn visit_u32<E: serde::de::Error>(self, v: u32) -> Result<Self::Value, E> {
            Ok(Some(v as usize))
        }
        fn visit_i8<E: serde::de::Error>(self, v: i8) -> Result<Self::Value, E> {
            Ok(Some(v.max(0) as usize))
        }
        fn visit_i16<E: serde::de::Error>(self, v: i16) -> Result<Self::Value, E> {
            Ok(Some(v.max(0) as usize))
        }
        fn visit_i32<E: serde::de::Error>(self, v: i32) -> Result<Self::Value, E> {
            Ok(Some(v.max(0) as usize))
        }
        fn visit_f32<E: serde::de::Error>(self, v: f32) -> Result<Self::Value, E> {
            Ok(Some((v as f64).round().max(0.0) as usize))
        }
    }
    d.deserialize_option(OptUsize)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointRef {
    Named(String),
    Coords(f64, f64),
}

impl PointRef {
    /// Reference a named point (mirrors Typst's `"A"` string refs).
    pub fn named(name: impl Into<String>) -> Self {
        PointRef::Named(name.into())
    }

    /// Reference a literal coordinate (mirrors Typst's `(x, y)` array refs).
    pub fn at(x: f64, y: f64) -> Self {
        PointRef::Coords(x, y)
    }

    pub fn resolve(&self, lookup: &HashMap<String, (f64, f64)>) -> Option<(f64, f64)> {
        match self {
            PointRef::Named(name) => lookup.get(name).copied(),
            PointRef::Coords(x, y) => Some((*x, *y)),
        }
    }
}

impl From<&str> for PointRef {
    fn from(s: &str) -> Self {
        PointRef::Named(s.to_string())
    }
}

impl From<String> for PointRef {
    fn from(s: String) -> Self {
        PointRef::Named(s)
    }
}

impl From<(f64, f64)> for PointRef {
    fn from(c: (f64, f64)) -> Self {
        PointRef::Coords(c.0, c.1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Object {
    #[serde(rename = "point")]
    Point {
        name: Option<String>,
        coords: (f64, f64),
        color: Option<Color>,
        size: Option<f64>,
    },
    #[serde(rename = "line")]
    Line {
        from: PointRef,
        to: PointRef,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "circle")]
    Circle {
        center: PointRef,
        radius: f64,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "polygon")]
    Polygon {
        points: Vec<PointRef>,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "ellipse")]
    Ellipse {
        center: PointRef,
        rx: f64,
        ry: f64,
        rotation: Option<f64>,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "arc")]
    Arc {
        center: PointRef,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "semicircle")]
    Semicircle {
        from: PointRef,
        to: PointRef,
        center: Option<PointRef>,
        dir: String,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    },
    #[serde(rename = "curve")]
    Curve {
        expr_str: String,
        t_min: Option<f64>,
        t_max: Option<f64>,
        var_name: String,
        #[serde(default, deserialize_with = "de_opt_usize")]
        samples: Option<usize>,
        #[serde(default)]
        tolerance: Option<f64>,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "curve_param")]
    CurveParam {
        x_expr: String,
        y_expr: String,
        t_min: Option<f64>,
        t_max: Option<f64>,
        #[serde(default, deserialize_with = "de_opt_usize")]
        samples: Option<usize>,
        #[serde(default)]
        tolerance: Option<f64>,
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "resolved_curve")]
    ResolvedCurve {
        points: Vec<(f64, f64)>,
        color: Option<Color>,
        stroke: Option<f64>,
    },
}

/// Direction of a semicircle bulge (mirrors Typst's `dir: "CW" / "CCW"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemicircleDir {
    Clockwise,
    CounterClockwise,
}

impl SemicircleDir {
    pub fn as_str(self) -> &'static str {
        match self {
            SemicircleDir::Clockwise => "cw",
            SemicircleDir::CounterClockwise => "ccw",
        }
    }

    /// Parse Typst-style direction names: `cw`, `ccw`, `clockwise`,
    /// `counterclockwise`, `counter-clockwise` (case-insensitive).
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "cw" | "clockwise" => Some(SemicircleDir::Clockwise),
            "ccw" | "counterclockwise" | "counter-clockwise" => {
                Some(SemicircleDir::CounterClockwise)
            }
            _ => None,
        }
    }
}

impl std::str::FromStr for SemicircleDir {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SemicircleDir::parse(s)
            .ok_or_else(|| format!("dir must be CW or CCW, got {s}"))
    }
}

// ---------------------------------------------------------------------------
// Typst-like free constructors (mirror `dgs-point`, `dgs-line`, …).
//
// Each constructor takes the same positional parameters as its Typst
// counterpart; optional Typst parameters (`color:`, `stroke:`, `fill:`,
// `size:`, …) default to `auto`/`none` and are set via the `with_*` methods,
// e.g. `circle("A", 2.0).with_color(color("red")).with_fill("#0000ff40")`.
// ---------------------------------------------------------------------------

/// Mirrors `dgs-point(name, x, y)`.
pub fn point(name: impl Into<String>, x: f64, y: f64) -> Object {
    Object::Point {
        name: Some(name.into()),
        coords: (x, y),
        color: None,
        size: None,
    }
}

/// An unnamed point.
pub fn point_at(x: f64, y: f64) -> Object {
    Object::Point {
        name: None,
        coords: (x, y),
        color: None,
        size: None,
    }
}

/// Mirrors `dgs-line(from, to)`. Both ends accept a name (`"A"`) or
/// coordinates (`(3.0, 4.0)`), like Typst's strings and arrays.
pub fn line(from: impl Into<PointRef>, to: impl Into<PointRef>) -> Object {
    Object::Line {
        from: from.into(),
        to: to.into(),
        color: None,
        stroke: None,
    }
}

/// Mirrors `dgs-circle(center, radius)`.
pub fn circle(center: impl Into<PointRef>, radius: f64) -> Object {
    Object::Circle {
        center: center.into(),
        radius,
        color: None,
        stroke: None,
        fill: None,
    }
}

/// Mirrors `dgs-polygon(..pts)`.
pub fn polygon(points: Vec<PointRef>) -> Object {
    Object::Polygon {
        points,
        color: None,
        stroke: None,
        fill: None,
    }
}

/// Mirrors `dgs-ellipse(center, rx, ry)` (`rotation` defaults to `0deg`,
/// settable via [`Object::with_rotation`]).
pub fn ellipse(center: impl Into<PointRef>, rx: f64, ry: f64) -> Object {
    Object::Ellipse {
        center: center.into(),
        rx,
        ry,
        rotation: None,
        color: None,
        stroke: None,
        fill: None,
    }
}

/// Mirrors the 4-argument `dgs-arc(center, radius, start-angle, end-angle)`
/// form (angles in degrees, like Typst's `deg` values).
pub fn arc(
    center: impl Into<PointRef>,
    radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> Object {
    Object::Arc {
        center: center.into(),
        radius,
        start_angle,
        end_angle,
        color: None,
        stroke: None,
    }
}

/// Mirrors `dgs-semicircle(from, to)` (`dir` defaults to CCW).
pub fn semicircle(from: impl Into<PointRef>, to: impl Into<PointRef>) -> Object {
    Object::Semicircle {
        from: from.into(),
        to: to.into(),
        center: None,
        dir: SemicircleDir::CounterClockwise.as_str().to_string(),
        color: None,
        stroke: None,
        fill: None,
    }
}

/// Mirrors `dgs-eq(expr)` (`var` defaults to `"x"`, range defaults to the
/// viewport's x-range at render time).
pub fn eq(expr: &str) -> Object {
    Object::Curve {
        expr_str: expr.to_string(),
        var_name: "x".to_string(),
        t_min: None,
        t_max: None,
        samples: None,
        tolerance: None,
        color: None,
        stroke: None,
    }
}

/// Mirrors `dgs-eq-param(x-expr, y-expr)` (`t` defaults to `0..2π`).
pub fn eq_param(x_expr: &str, y_expr: &str) -> Object {
    Object::CurveParam {
        x_expr: x_expr.to_string(),
        y_expr: y_expr.to_string(),
        t_min: None,
        t_max: None,
        samples: None,
        tolerance: None,
        color: None,
        stroke: None,
    }
}

/// Parse a color from a name (`"red"`) or hex string (`"#ff0000"`, `#f00`,
/// `#ff000080`). Panics on invalid input, like Typst does.
pub fn color(s: &str) -> Color {
    Color::parse(s).unwrap_or_else(|| panic!("invalid color: {s}"))
}

impl Object {
    /// Set `color` (mirrors Typst's `color:` parameter).
    pub fn with_color(mut self, color: impl Into<Color>) -> Self {
        let color = color.into();
        match &mut self {
            Object::Point { color: c, .. }
            | Object::Line { color: c, .. }
            | Object::Arc { color: c, .. } => *c = Some(color),
            Object::Circle { color: c, .. }
            | Object::Polygon { color: c, .. }
            | Object::Ellipse { color: c, .. }
            | Object::Semicircle { color: c, .. } => *c = Some(color),
            Object::Curve { color: c, .. }
            | Object::CurveParam { color: c, .. }
            | Object::ResolvedCurve { color: c, .. } => *c = Some(color),
        }
        self
    }

    /// Set `stroke` width (mirrors Typst's `stroke:` parameter).
    pub fn with_stroke(mut self, stroke: f64) -> Self {
        match &mut self {
            Object::Line { stroke: s, .. } | Object::Arc { stroke: s, .. } => {
                *s = Some(stroke)
            }
            Object::Circle { stroke: s, .. }
            | Object::Polygon { stroke: s, .. }
            | Object::Ellipse { stroke: s, .. }
            | Object::Semicircle { stroke: s, .. } => *s = Some(stroke),
            Object::Curve { stroke: s, .. }
            | Object::CurveParam { stroke: s, .. }
            | Object::ResolvedCurve { stroke: s, .. } => *s = Some(stroke),
            Object::Point { .. } => {}
        }
        self
    }

    /// Set `fill` color (mirrors Typst's `fill:` parameter).
    pub fn with_fill(mut self, fill: impl Into<Color>) -> Self {
        let fill = fill.into();
        match &mut self {
            Object::Circle { fill: f, .. }
            | Object::Polygon { fill: f, .. }
            | Object::Ellipse { fill: f, .. }
            | Object::Semicircle { fill: f, .. } => *f = Some(fill),
            _ => {}
        }
        self
    }

    /// Set point `size` (mirrors Typst's `size:` parameter).
    pub fn with_size(mut self, size: f64) -> Self {
        if let Object::Point { size: s, .. } = &mut self {
            *s = Some(size);
        }
        self
    }

    /// Set ellipse `rotation` in degrees (mirrors `rotation: 0deg`).
    pub fn with_rotation(mut self, degrees: f64) -> Self {
        if let Object::Ellipse { rotation, .. } = &mut self {
            *rotation = Some(degrees);
        }
        self
    }

    /// Set semicircle `dir` (mirrors `dir: "CCW"`).
    pub fn with_dir(mut self, dir: SemicircleDir) -> Self {
        if let Object::Semicircle { dir: d, .. } = &mut self {
            *d = dir.as_str().to_string();
        }
        self
    }

    /// Set an explicit semicircle `center` (mirrors `center:`).
    pub fn with_center(mut self, center: impl Into<PointRef>) -> Self {
        if let Object::Semicircle { center: c, .. } = &mut self {
            *c = Some(center.into());
        }
        self
    }

    /// Set the curve variable name (mirrors `var: "x"`).
    pub fn with_var(mut self, var: &str) -> Self {
        if let Object::Curve { var_name, .. } = &mut self {
            *var_name = var.to_string();
        }
        self
    }

    /// Set the curve sampling range (mirrors `t1:`/`t2:` on `dgs-eq-param`
    /// and the `in [a, b]` DSL syntax).
    pub fn with_range(mut self, t_min: f64, t_max: f64) -> Self {
        match &mut self {
            Object::Curve {
                t_min: lo,
                t_max: hi,
                ..
            }
            | Object::CurveParam {
                t_min: lo,
                t_max: hi,
                ..
            } => {
                *lo = Some(t_min);
                *hi = Some(t_max);
            }
            _ => {}
        }
        self
    }

    /// Set the base sample count for curve sampling (default 256).
    ///
    /// Higher values give smoother curves; adaptive subdivision (guided by
    /// a numerical derivative / midpoint-deviation estimate) then refines
    /// high-curvature spans up to `8 * samples` points.
    pub fn with_samples(mut self, n: usize) -> Self {
        match &mut self {
            Object::Curve { samples, .. } | Object::CurveParam { samples, .. } => {
                *samples = Some(n.max(2));
            }
            _ => {}
        }
        self
    }

    /// Set the adaptive-subdivision tolerance in math units (default:
    /// viewport height * 0.001). Smaller values subdivide more aggressively.
    pub fn with_tolerance(mut self, tol: f64) -> Self {
        match &mut self {
            Object::Curve { tolerance, .. } | Object::CurveParam { tolerance, .. } => {
                *tolerance = Some(tol);
            }
            _ => {}
        }
        self
    }

    /// Alias for [`Object::with_samples`].
    pub fn with_precision(mut self, n: usize) -> Self {
        match &mut self {
            Object::Curve { samples, .. } | Object::CurveParam { samples, .. } => {
                *samples = Some(n.max(2));
            }
            _ => {}
        }
        self
    }
}

pub fn build_point_lookup(objects: &[Object]) -> HashMap<String, (f64, f64)> {
    let mut lookup = HashMap::new();
    for obj in objects {
        if let Object::Point {
            name: Some(name),
            coords,
            ..
        } = obj
        {
            lookup.insert(name.clone(), *coords);
        }
    }
    lookup
}

pub fn resolve_objects(
    objects: &[Object],
    lookup: &HashMap<String, (f64, f64)>,
    viewport: &crate::viewport::Viewport,
) -> Vec<Object> {
    let mut resolved = Vec::with_capacity(objects.len());
    for obj in objects {
        match obj {
            Object::Point { .. } => {
                resolved.push(obj.clone());
            }
            Object::Line {
                from,
                to,
                color,
                stroke,
            } => {
                resolved.push(Object::Line {
                    from: resolve_point_ref(from, lookup),
                    to: resolve_point_ref(to, lookup),
                    color: *color,
                    stroke: *stroke,
                });
            }
            Object::Circle {
                center,
                radius,
                color,
                stroke,
                fill,
            } => {
                resolved.push(Object::Circle {
                    center: resolve_point_ref(center, lookup),
                    radius: *radius,
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            Object::Polygon {
                points,
                color,
                stroke,
                fill,
            } => {
                resolved.push(Object::Polygon {
                    points: points
                        .iter()
                        .map(|p| resolve_point_ref(p, lookup))
                        .collect(),
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            Object::Ellipse {
                center,
                rx,
                ry,
                rotation,
                color,
                stroke,
                fill,
            } => {
                resolved.push(Object::Ellipse {
                    center: resolve_point_ref(center, lookup),
                    rx: *rx,
                    ry: *ry,
                    rotation: *rotation,
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            Object::Arc {
                center,
                radius,
                start_angle,
                end_angle,
                color,
                stroke,
            } => {
                resolved.push(Object::Arc {
                    center: resolve_point_ref(center, lookup),
                    radius: *radius,
                    start_angle: *start_angle,
                    end_angle: *end_angle,
                    color: *color,
                    stroke: *stroke,
                });
            }
            Object::Semicircle {
                from,
                to,
                center,
                dir,
                color,
                stroke,
                fill,
            } => {
                resolved.push(Object::Semicircle {
                    from: resolve_point_ref(from, lookup),
                    to: resolve_point_ref(to, lookup),
                    center: center.as_ref().map(|c| resolve_point_ref(c, lookup)),
                    dir: dir.clone(),
                    color: *color,
                    stroke: *stroke,
                    fill: *fill,
                });
            }
            Object::Curve {
                expr_str,
                t_min,
                t_max,
                var_name,
                samples,
                tolerance,
                color,
                stroke,
            } => {
                // Fix #1: with no explicit range, sample the viewport's
                // x-range instead of a hardcoded -10..10.
                let lo = t_min.unwrap_or(viewport.x1);
                let hi = t_max.unwrap_or(viewport.x2);
                let points = evaluate_curve(
                    expr_str,
                    var_name,
                    lo,
                    hi,
                    samples.unwrap_or(DEFAULT_SAMPLES),
                    tolerance.unwrap_or_else(|| default_tolerance(viewport)),
                    viewport,
                    lookup,
                );
                resolved.push(Object::ResolvedCurve {
                    points,
                    color: *color,
                    stroke: *stroke,
                });
            }
            Object::CurveParam {
                x_expr,
                y_expr,
                t_min,
                t_max,
                samples,
                tolerance,
                color,
                stroke,
            } => {
                let points = evaluate_parametric(
                    x_expr,
                    y_expr,
                    t_min.unwrap_or(0.0),
                    t_max.unwrap_or(std::f64::consts::TAU),
                    samples.unwrap_or(DEFAULT_SAMPLES),
                    tolerance.unwrap_or_else(|| default_tolerance(viewport)),
                    viewport,
                );
                resolved.push(Object::ResolvedCurve {
                    points,
                    color: *color,
                    stroke: *stroke,
                });
            }
            Object::ResolvedCurve { .. } => {
                resolved.push(obj.clone());
            }
        }
    }
    resolved
}

fn resolve_point_ref(pr: &PointRef, lookup: &HashMap<String, (f64, f64)>) -> PointRef {
    match pr {
        PointRef::Named(name) => match lookup.get(name) {
            Some(coords) => PointRef::Coords(coords.0, coords.1),
            None => PointRef::Named(name.clone()),
        },
        PointRef::Coords(x, y) => PointRef::Coords(*x, *y),
    }
}

/// Default base sample count (override per curve via `with_samples`).
pub const DEFAULT_SAMPLES: usize = 256;
/// Max adaptive-subdivision depth per base interval.
const MAX_DEPTH: u32 = 10;

fn default_tolerance(viewport: &crate::viewport::Viewport) -> f64 {
    ((viewport.y2 - viewport.y1).abs() * 0.001).max(1e-9)
}

fn eval_y(
    expr: &crate::parser::Expr,
    var_name: &str,
    t: f64,
) -> Option<f64> {
    match crate::parser::eval(expr, &[(var_name, t)]) {
        Ok(y) if y.is_finite() => Some(y),
        _ => None,
    }
}

/// Break threshold: a jump larger than this is treated as a discontinuity
/// (asymptote), not a steep-but-continuous slope. Fix #2: previously the
/// polyline was drawn straight across asymptotes, producing vertical
/// streaks / "random bumps" on e.g. linear-fractional functions.
fn jump_threshold(viewport: &crate::viewport::Viewport) -> f64 {
    ((viewport.y2 - viewport.y1).abs() * 4.0).max(1e-6)
}

fn subdivide(
    expr: &crate::parser::Expr,
    var_name: &str,
    t0: f64,
    y0: f64,
    t1: f64,
    y1: f64,
    tol: f64,
    jump: f64,
    depth: u32,
    out: &mut Vec<(f64, f64)>,
) {
    let tm = 0.5 * (t0 + t1);
    let Some(ym) = eval_y(expr, var_name, tm) else {
        // Non-finite midpoint: discontinuity inside -> break the polyline.
        out.push((t0, y0));
        out.push((f64::NAN, f64::NAN));
        out.push((t1, y1));
        return;
    };
    // Derivative-guided discontinuity check: if the midpoint is wildly off
    // the chord (or the chord itself jumps), this is an asymptote.
    let chord = 0.5 * (y0 + y1);
    if (y1 - y0).abs() > jump && (ym - chord).abs() > jump * 0.25 {
        out.push((t0, y0));
        out.push((f64::NAN, f64::NAN));
        out.push((t1, y1));
        return;
    }
    if depth >= MAX_DEPTH || (ym - chord).abs() <= tol {
        out.push((t0, y0));
        return;
    }
    subdivide(expr, var_name, t0, y0, tm, ym, tol, jump, depth + 1, out);
    subdivide(expr, var_name, tm, ym, t1, y1, tol, jump, depth + 1, out);
}

fn evaluate_curve(
    expr_str: &str,
    var_name: &str,
    t_min: f64,
    t_max: f64,
    samples: usize,
    tol: f64,
    viewport: &crate::viewport::Viewport,
    _lookup: &HashMap<String, (f64, f64)>,
) -> Vec<(f64, f64)> {
    let expr = match crate::parser::parse(expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    if !(t_min.is_finite() && t_max.is_finite()) || t_max <= t_min {
        return Vec::new();
    }
    let base = samples.max(2);
    let jump = jump_threshold(viewport);
    // Hard cap so adversarial expressions can't blow up memory.
    let cap = (base * 16).max(64);

    let mut grid: Vec<Option<f64>> = Vec::with_capacity(base + 1);
    for i in 0..=base {
        let t = t_min + (t_max - t_min) * i as f64 / base as f64;
        grid.push(eval_y(&expr, var_name, t));
    }

    let mut out: Vec<(f64, f64)> = Vec::with_capacity(base + 1);
    for i in 0..base {
        if out.len() > cap {
            break;
        }
        let t0 = t_min + (t_max - t_min) * i as f64 / base as f64;
        let t1 = t_min + (t_max - t_min) * (i + 1) as f64 / base as f64;
        match (grid[i], grid[i + 1]) {
            (Some(y0), Some(y1)) => {
                // Fast path: straight (e.g. linear) spans emit no extra
                // points — the midpoint lies on the chord, so no bumps.
                let tm = 0.5 * (t0 + t1);
                match eval_y(&expr, var_name, tm) {
                    None => {
                        out.push((t0, y0));
                        out.push((f64::NAN, f64::NAN));
                    }
                    Some(ym) => {
                        let chord = 0.5 * (y0 + y1);
                        if (y1 - y0).abs() > jump && (ym - chord).abs() > jump * 0.25 {
                            out.push((t0, y0));
                            out.push((f64::NAN, f64::NAN));
                        } else if (ym - chord).abs() <= tol {
                            out.push((t0, y0));
                        } else {
                            let mut seg = Vec::new();
                            subdivide(
                                &expr, var_name, t0, y0, tm, ym, tol, jump, 1, &mut seg,
                            );
                            out.extend(seg);
                            let mut seg2 = Vec::new();
                            subdivide(
                                &expr, var_name, tm, ym, t1, y1, tol, jump, 1, &mut seg2,
                            );
                            out.extend(seg2);
                        }
                    }
                }
            }
            (Some(y0), None) | (None, Some(y0)) => {
                let t = if grid[i].is_some() { t0 } else { t1 };
                out.push((t, y0));
                out.push((f64::NAN, f64::NAN));
            }
            (None, None) => {}
        }
    }
    // Push final valid grid point.
    for i in (0..=base).rev() {
        if let Some(y) = grid[i] {
            let t = t_min + (t_max - t_min) * i as f64 / base as f64;
            out.push((t, y));
            break;
        }
    }
    if out.len() > cap {
        out.truncate(cap);
    }
    out
}

fn evaluate_parametric(
    x_expr_str: &str,
    y_expr_str: &str,
    t_min: f64,
    t_max: f64,
    samples: usize,
    tol: f64,
    viewport: &crate::viewport::Viewport,
) -> Vec<(f64, f64)> {
    let x_expr = match crate::parser::parse(x_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let y_expr = match crate::parser::parse(y_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    if !(t_min.is_finite() && t_max.is_finite()) || t_max <= t_min {
        return Vec::new();
    }
    let base = samples.max(2);
    let cap = (base * 16).max(64);
    let scale = (viewport.x2 - viewport.x1).abs().max((viewport.y2 - viewport.y1).abs()).max(1e-9);
    let jump = scale * 4.0;

    let eval_pt = |t: f64| -> Option<(f64, f64)> {
        match (
            crate::parser::eval(&x_expr, &[("t", t)]),
            crate::parser::eval(&y_expr, &[("t", t)]),
        ) {
            (Ok(x), Ok(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
            _ => None,
        }
    };

    let mut out: Vec<(f64, f64)> = Vec::with_capacity(base + 1);
    let mut prev_t = t_min;
    let mut prev_p = eval_pt(t_min);
    // Adaptive midpoint subdivision using the numerical-deviation estimate.
    let mut stack: Vec<(f64, Option<(f64, f64)>, f64, Option<(f64, f64)>, u32)> = Vec::new();
    for i in 0..base {
        let t1 = t_min + (t_max - t_min) * (i + 1) as f64 / base as f64;
        let p1 = eval_pt(t1);
        stack.push((prev_t, prev_p, t1, p1, 0));
        prev_t = t1;
        prev_p = p1;
    }
    // Process in order: collect refined points per interval.
    let mut ordered: Vec<(f64, Option<(f64, f64)>)> = Vec::new();
    ordered.push((t_min, eval_pt(t_min)));
    // Simple approach: iterate base intervals in order, refining each.
    ordered.clear();
    let mut ts: Vec<f64> = (0..=base)
        .map(|i| t_min + (t_max - t_min) * i as f64 / base as f64)
        .collect();
    // Refine pass: insert midpoints where deviation exceeds tol.
    for _ in 0..MAX_DEPTH {
        let mut inserted = false;
        let mut j = 0;
        while j + 1 < ts.len() && ts.len() < cap {
            let (a, b) = (ts[j], ts[j + 1]);
            let (pa, pb) = (eval_pt(a), eval_pt(b));
            let m = 0.5 * (a + b);
            let pm = eval_pt(m);
            let need = match (pa, pb, pm) {
                (Some(pa), Some(pb), Some(pm)) => {
                    let chord = (0.5 * (pa.0 + pb.0), 0.5 * (pa.1 + pb.1));
                    let dev = ((pm.0 - chord.0).powi(2) + (pm.1 - chord.1).powi(2)).sqrt();
                    let seg = ((pb.0 - pa.0).powi(2) + (pb.1 - pa.1).powi(2)).sqrt();
                    // Derivative-guided break on teleporting spans.
                    if seg > jump {
                        false
                    } else {
                        dev > tol
                    }
                }
                _ => false,
            };
            if need {
                ts.insert(j + 1, m);
                inserted = true;
                j += 2;
            } else {
                j += 1;
            }
        }
        if !inserted {
            break;
        }
    }
    let mut prev_valid = false;
    for t in ts {
        match eval_pt(t) {
            Some(p) => {
                // Break across teleporting jumps (derivative check).
                if let Some(last) = out.last().filter(|_| prev_valid) {
                    let d = ((p.0 - last.0).powi(2) + (p.1 - last.1).powi(2)).sqrt();
                    if d > jump {
                        out.push((f64::NAN, f64::NAN));
                    }
                }
                out.push(p);
                prev_valid = true;
            }
            None => {
                if prev_valid {
                    out.push((f64::NAN, f64::NAN));
                }
                prev_valid = false;
            }
        }
        if out.len() >= cap {
            break;
        }
    }
    let _ = stack;
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_lookup() {
        let objects = vec![Object::Point {
            name: Some("A".to_string()),
            coords: (1.0, 2.0),
            color: None,
            size: None,
        }];
        let lookup = build_point_lookup(&objects);
        assert_eq!(lookup.get("A"), Some(&(1.0, 2.0)));
    }

    #[test]
    fn test_resolve_named_point() {
        let pr = PointRef::Named("A".to_string());
        let mut lookup = HashMap::new();
        lookup.insert("A".to_string(), (3.0, 4.0));
        assert_eq!(pr.resolve(&lookup), Some((3.0, 4.0)));
    }

    #[test]
    fn test_resolve_curve() {
        let vp = crate::viewport::Viewport::new(-10.0, -10.0, 10.0, 10.0, 200.0, 200.0);
        let points = evaluate_curve("x^2", "x", 0.0, 2.0, DEFAULT_SAMPLES, default_tolerance(&vp), &vp, &HashMap::new());
        assert!(!points.is_empty());
        assert!((points[0].1 - 0.0).abs() < 1e-10);
    }
}
