use crate::color::Color;
use crate::objects::{Object, PointRef, SemicircleDir};
use crate::theme::Theme;
use crate::viewport::Viewport;

/// A dynamic-geometry scene: math-coordinate geometry that renders to SVG,
/// PNG, or any future format via [`Renderer`].
///
/// Geometry is described in math coordinates; rendering samples
/// expression-driven curves via [`crate::parser`].
///
/// ```
/// let svg = dgs::Scene::new(dgs::Viewport::new(-5.0, -5.0, 5.0, 5.0, 400.0, 400.0))
///     .grid(true)
///     .axes(true)
///     .point_named("A", 0.0, 0.0)
///     .point_named("B", 3.0, 4.0)
///     .line("A", "B")
///     .circle("A", 2.0)
///     .to_svg();
/// assert!(svg.starts_with("<svg"));
/// ```
#[derive(Debug, Clone)]
pub struct Scene {
    viewport: Viewport,
    theme: Theme,
    grid: bool,
    grid_color: Option<Color>,
    grid_width: Option<f64>,
    grid_spacing: Option<f64>,
    axes: bool,
    axis_color: Option<Color>,
    axis_width: Option<f64>,
    axis_label_size: Option<f64>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new(viewport: Viewport) -> Self {
        Scene {
            viewport,
            theme: Theme::default(),
            grid: false,
            grid_color: None,
            grid_width: None,
            grid_spacing: None,
            axes: false,
            axis_color: None,
            axis_width: None,
            axis_label_size: None,
            objects: Vec::new(),
        }
    }

    pub fn viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = viewport;
        self
    }

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn grid(mut self, grid: bool) -> Self {
        self.grid = grid;
        self
    }

    pub fn grid_color(mut self, color: Color) -> Self {
        self.grid_color = Some(color);
        self
    }

    pub fn grid_width(mut self, width: f64) -> Self {
        self.grid_width = Some(width);
        self
    }

    pub fn grid_spacing(mut self, spacing: f64) -> Self {
        self.grid_spacing = Some(spacing);
        self
    }

    pub fn axes(mut self, axes: bool) -> Self {
        self.axes = axes;
        self
    }

    pub fn axis_color(mut self, color: Color) -> Self {
        self.axis_color = Some(color);
        self
    }

    pub fn axis_width(mut self, width: f64) -> Self {
        self.axis_width = Some(width);
        self
    }

    pub fn axis_label_size(mut self, size: f64) -> Self {
        self.axis_label_size = Some(size);
        self
    }

    /// Define a named point that later objects can reference by name.
    pub fn point_named(mut self, name: impl Into<String>, x: f64, y: f64) -> Self {
        self.objects.push(Object::Point {
            name: Some(name.into()),
            coords: (x, y),
            color: None,
            size: None,
        });
        self
    }

    /// Define an unnamed (decorative) point.
    pub fn point_at(mut self, x: f64, y: f64) -> Self {
        self.objects.push(Object::Point {
            name: None,
            coords: (x, y),
            color: None,
            size: None,
        });
        self
    }

    /// A line between two point references (names or coordinates).
    pub fn line(mut self, from: impl Into<PointRef>, to: impl Into<PointRef>) -> Self {
        self.objects.push(Object::Line {
            from: from.into(),
            to: to.into(),
            color: None,
            stroke: None,
        });
        self
    }

    pub fn circle(mut self, center: impl Into<PointRef>, radius: f64) -> Self {
        self.objects.push(Object::Circle {
            center: center.into(),
            radius,
            color: None,
            stroke: None,
            fill: None,
        });
        self
    }

    pub fn polygon(mut self, points: Vec<PointRef>) -> Self {
        self.objects.push(Object::Polygon {
            points,
            color: None,
            stroke: None,
            fill: None,
        });
        self
    }

    pub fn ellipse(
        mut self,
        center: impl Into<PointRef>,
        rx: f64,
        ry: f64,
        rotation: Option<f64>,
    ) -> Self {
        self.objects.push(Object::Ellipse {
            center: center.into(),
            rx,
            ry,
            rotation,
            color: None,
            stroke: None,
            fill: None,
        });
        self
    }

    pub fn arc(
        mut self,
        center: impl Into<PointRef>,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> Self {
        self.objects.push(Object::Arc {
            center: center.into(),
            radius,
            start_angle,
            end_angle,
            color: None,
            stroke: None,
        });
        self
    }

    pub fn semicircle(
        mut self,
        from: impl Into<PointRef>,
        to: impl Into<PointRef>,
        center: Option<PointRef>,
        dir: SemicircleDir,
    ) -> Self {
        self.objects.push(Object::Semicircle {
            from: from.into(),
            to: to.into(),
            center,
            dir: dir.as_str().to_string(),
            color: None,
            stroke: None,
            fill: None,
        });
        self
    }

    /// Plot `y = f(var)` where `var` is evaluated over `[t_min, t_max]`.
    ///
    /// The expression uses the [`crate::parser`] DSL, e.g. `"sin(x) / x"`.
    pub fn curve(mut self, expr: &str, var: &str, t_min: f64, t_max: f64) -> Self {
        self.objects.push(Object::Curve {
            expr_str: expr.to_string(),
            var_name: var.to_string(),
            t_min: Some(t_min),
            t_max: Some(t_max),
            samples: None,
            tolerance: None,
            color: None,
            stroke: None,
        });
        self
    }

    /// Plot `y = f(var)` with explicit precision controls: `samples` is the
    /// base sample count and `tolerance` the adaptive-subdivision tolerance
    /// in math units (pass `None` for defaults).
    pub fn curve_with(
        mut self,
        expr: &str,
        var: &str,
        t_min: f64,
        t_max: f64,
        samples: Option<usize>,
        tolerance: Option<f64>,
    ) -> Self {
        self.objects.push(Object::Curve {
            expr_str: expr.to_string(),
            var_name: var.to_string(),
            t_min: Some(t_min),
            t_max: Some(t_max),
            samples,
            tolerance,
            color: None,
            stroke: None,
        });
        self
    }

    /// Plot a parametric curve `(x(t), y(t))` over `[t_min, t_max]`.
    pub fn curve_param(mut self, x_expr: &str, y_expr: &str, t_min: f64, t_max: f64) -> Self {
        self.objects.push(Object::CurveParam {
            x_expr: x_expr.to_string(),
            y_expr: y_expr.to_string(),
            t_min: Some(t_min),
            t_max: Some(t_max),
            samples: None,
            tolerance: None,
            color: None,
            stroke: None,
        });
        self
    }

    /// Plot a parametric curve with explicit precision controls.
    pub fn curve_param_with(
        mut self,
        x_expr: &str,
        y_expr: &str,
        t_min: f64,
        t_max: f64,
        samples: Option<usize>,
        tolerance: Option<f64>,
    ) -> Self {
        self.objects.push(Object::CurveParam {
            x_expr: x_expr.to_string(),
            y_expr: y_expr.to_string(),
            t_min: Some(t_min),
            t_max: Some(t_max),
            samples,
            tolerance,
            color: None,
            stroke: None,
        });
        self
    }

    /// Append a raw, pre-resolved polyline.
    pub fn polyline(mut self, points: Vec<(f64, f64)>) -> Self {
        self.objects.push(Object::ResolvedCurve {
            points,
            color: None,
            stroke: None,
        });
        self
    }

    /// Append a raw [`Object`], e.g. one deserialized from an external
    /// wire format.
    pub fn push(mut self, obj: Object) -> Self {
        self.objects.push(obj);
        self
    }

    /// Append many objects at once, e.g. from an iterator over an outside
    /// vector: `scene.extend(pts.iter().map(|(x, y)| point_at(*x, *y)))`.
    pub fn extend(mut self, objs: impl IntoIterator<Item = Object>) -> Self {
        self.objects.extend(objs);
        self
    }

    /// Style the most recently added object: `color`, `stroke`, `fill`.
    pub fn style_last(
        mut self,
        color: Option<Color>,
        stroke: Option<f64>,
        fill: Option<Color>,
    ) -> Self {
        let Some(obj) = self.objects.last_mut() else {
            return self;
        };
        match obj {
            Object::Point { color: c, .. } => *c = color,
            Object::Line { color: c, stroke: s, .. }
            | Object::Arc { color: c, stroke: s, .. } => {
                *c = color;
                *s = stroke;
            }
            Object::Circle { color: c, stroke: s, fill: f, .. }
            | Object::Polygon { color: c, stroke: s, fill: f, .. }
            | Object::Ellipse { color: c, stroke: s, fill: f, .. }
            | Object::Semicircle { color: c, stroke: s, fill: f, .. } => {
                *c = color;
                *s = stroke;
                *f = fill;
            }
            Object::Curve { color: c, stroke: s, .. }
            | Object::CurveParam { color: c, stroke: s, .. }
            | Object::ResolvedCurve { color: c, stroke: s, .. } => {
                *c = color;
                *s = stroke;
            }
        }
        self
    }

    /// Render the canvas to an SVG string.
    pub fn to_svg(&self) -> String {
        let viewport = &self.viewport;
        let theme = &self.theme;
        let mut svg = crate::svg::SvgBuilder::new(viewport.width, viewport.height);

        // 1. Background
        svg.rect(
            0.0,
            0.0,
            viewport.width,
            viewport.height,
            theme.background(),
            None,
        );

        // 2. Grid
        if self.grid {
            crate::grid::render_grid(
                viewport,
                self.grid_color.unwrap_or_else(|| theme.grid_color()),
                self.grid_width.unwrap_or(0.5),
                self.grid_spacing.unwrap_or(1.0),
                &mut svg,
            );
        }

        // 3. Build point lookup and resolve objects
        let lookup = crate::objects::build_point_lookup(&self.objects);
        let resolved = crate::objects::resolve_objects(&self.objects, &lookup, viewport);

        // 4. Axes
        if self.axes {
            crate::axis::render_axes(
                viewport,
                self.axis_color.unwrap_or_else(|| theme.axis_color()),
                self.axis_width.unwrap_or(1.5),
                theme.axis_label_color(),
                self.axis_label_size.unwrap_or(10.0),
                &mut svg,
            );
        }

        // 5. Render all objects
        let default_color = theme.text_color();
        for obj in &resolved {
            match obj {
                crate::objects::Object::Point {
                    coords,
                    color,
                    size,
                    ..
                } => {
                    crate::point::render_point(
                        *coords,
                        color.unwrap_or(default_color),
                        size.unwrap_or(crate::point::DEFAULT_POINT_SIZE),
                        viewport,
                        &mut svg,
                    );
                }
                crate::objects::Object::Line {
                    from,
                    to,
                    color,
                    stroke,
                } => {
                    if let (Some(f), Some(t)) = (from.resolve(&lookup), to.resolve(&lookup)) {
                        crate::line::render_line(
                            f,
                            t,
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(crate::line::DEFAULT_STROKE),
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::Circle {
                    center,
                    radius,
                    color,
                    stroke,
                    fill,
                } => {
                    if let Some(c) = center.resolve(&lookup) {
                        crate::circle::render_circle(
                            c,
                            *radius,
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(1.5),
                            *fill,
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::Polygon {
                    points,
                    color,
                    stroke,
                    fill,
                } => {
                    let resolved_points: Vec<(f64, f64)> =
                        points.iter().filter_map(|p| p.resolve(&lookup)).collect();
                    if !resolved_points.is_empty() {
                        crate::polygon::render_polygon(
                            &resolved_points,
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(1.5),
                            *fill,
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::Ellipse {
                    center,
                    rx,
                    ry,
                    rotation,
                    color,
                    stroke,
                    fill,
                } => {
                    if let Some(c) = center.resolve(&lookup) {
                        crate::ellipse::render_ellipse(
                            c,
                            *rx,
                            *ry,
                            rotation.unwrap_or(0.0),
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(1.5),
                            *fill,
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                    color,
                    stroke,
                } => {
                    if let Some(c) = center.resolve(&lookup) {
                        crate::arc::render_arc(
                            c,
                            *radius,
                            *start_angle,
                            *end_angle,
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(1.5),
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::Semicircle {
                    from,
                    to,
                    center,
                    dir,
                    color,
                    stroke,
                    fill,
                } => {
                    if let (Some(f), Some(t)) = (from.resolve(&lookup), to.resolve(&lookup)) {
                        let c = center.as_ref().and_then(|c| c.resolve(&lookup));
                        crate::semicircle::render_semicircle(
                            f,
                            t,
                            c,
                            dir,
                            color.unwrap_or(default_color),
                            stroke.unwrap_or(1.5),
                            *fill,
                            viewport,
                            &mut svg,
                        );
                    }
                }
                crate::objects::Object::ResolvedCurve {
                    points,
                    color,
                    stroke,
                } => {
                    crate::curve::render_curve(
                        points,
                        color.unwrap_or(default_color),
                        stroke.unwrap_or(1.5),
                        viewport,
                        &mut svg,
                    );
                }
                crate::objects::Object::Curve { .. } => {
                    // Unresolved curves should have been resolved by resolve_objects
                }
                crate::objects::Object::CurveParam { .. } => {
                    // Unresolved parametric curves should have been resolved by resolve_objects
                }
            }
        }

        svg.build()
    }

    /// Render the scene to bytes in the requested [`Format`].
    pub fn render(&self, format: Format) -> Result<Vec<u8>, RenderError> {
        match format {
            Format::Svg => Ok(self.to_svg().into_bytes()),
            #[cfg(feature = "png")]
            Format::Png { scale } => self.to_png(scale),
        }
    }

    /// Render with any [`Renderer`] implementation (the extension point for
    /// future output formats).
    pub fn render_with<R: Renderer>(&self, renderer: R) -> Result<R::Output, R::Error> {
        renderer.render(self)
    }

    /// Rasterize the scene to PNG bytes at `scale` times the viewport's
    /// pixel size (e.g. `1.0` for native size, `2.0` for HiDPI).
    #[cfg(feature = "png")]
    pub fn to_png(&self, scale: f64) -> Result<Vec<u8>, RenderError> {
        PngRenderer::new(scale).render(self)
    }
}

/// Output format selector for [`Scene::render`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Svg,
    #[cfg(feature = "png")]
    Png {
        scale: f64,
    },
}

/// Errors that can occur while rendering a [`Scene`].
#[derive(Debug)]
pub enum RenderError {
    Svg(String),
    Pixmap,
    Png(String),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::Svg(e) => write!(f, "invalid SVG for rasterization: {e}"),
            RenderError::Pixmap => write!(f, "failed to allocate pixel buffer"),
            RenderError::Png(e) => write!(f, "PNG encoding failed: {e}"),
        }
    }
}

impl std::error::Error for RenderError {}

/// Extension point for output formats: implement this to add a renderer
/// (e.g. PDF, canvas, …) without touching [`Scene`].
pub trait Renderer {
    type Output;
    type Error;

    fn render(&self, scene: &Scene) -> Result<Self::Output, Self::Error>;
}

/// The built-in SVG renderer.
#[derive(Debug, Clone, Copy, Default)]
pub struct SvgRenderer;

impl Renderer for SvgRenderer {
    type Output = String;
    type Error = std::convert::Infallible;

    fn render(&self, scene: &Scene) -> Result<String, std::convert::Infallible> {
        Ok(scene.to_svg())
    }
}

/// The built-in PNG renderer (requires the `png` feature).
#[cfg(feature = "png")]
#[derive(Debug, Clone, Copy)]
pub struct PngRenderer {
    scale: f64,
}

#[cfg(feature = "png")]
impl PngRenderer {
    pub fn new(scale: f64) -> Self {
        Self { scale }
    }
}

#[cfg(feature = "png")]
impl Renderer for PngRenderer {
    type Output = Vec<u8>;
    type Error = RenderError;

    fn render(&self, scene: &Scene) -> Result<Vec<u8>, RenderError> {
        use resvg::tiny_skia::{Pixmap, Transform};
        use resvg::usvg::{Options, Tree};

        let svg = scene.to_svg();
        let tree = Tree::from_str(&svg, &Options::default())
            .map_err(|e| RenderError::Svg(e.to_string()))?;
        let size = tree.size();
        let width = ((size.width() as f64 * self.scale).round() as u32).max(1);
        let height = ((size.height() as f64 * self.scale).round() as u32).max(1);
        let mut pixmap = Pixmap::new(width, height).ok_or(RenderError::Pixmap)?;
        resvg::render(
            &tree,
            Transform::from_scale(self.scale as f32, self.scale as f32),
            &mut pixmap.as_mut(),
        );
        pixmap.encode_png().map_err(|e| RenderError::Png(e.to_string()))
    }
}

/// Create a scene with the same defaults as Typst's `dgs-canvas`
/// (`x1: 0, y1: 0, x2: 10, y2: 10`, 300×300, light theme, grid + axes on).
pub fn scene() -> Scene {
    Scene::new(Viewport::new(0.0, 0.0, 10.0, 10.0, 300.0, 300.0))
        .grid(true)
        .axes(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn test_render_basic() {
        let svg = Scene::new(Viewport::new(-5.0, -5.0, 5.0, 5.0, 200.0, 200.0))
            .grid(true)
            .axes(true)
            .point_named("A", 0.0, 0.0)
            .point_named("B", 3.0, 4.0)
            .line("A", "B")
            .circle("A", 2.0)
            .curve("sin(x)", "x", -5.0, 5.0)
            .style_last(Some(Color::rgb(255, 0, 0)), Some(2.0), None)
            .to_svg();
        assert!(svg.starts_with("<svg"));
    }

    #[test]
    fn test_point_ref_conversions() {
        let _: PointRef = "A".into();
        let _: PointRef = (1.0, 2.0).into();
    }

    #[test]
    fn test_render_format_svg() {
        let bytes = scene().render(Format::Svg).unwrap();
        assert!(bytes.starts_with(b"<svg"));
    }

    #[cfg(feature = "png")]
    #[test]
    fn test_render_png() {
        let bytes = scene().to_png(1.0).unwrap();
        // PNG magic bytes
        assert_eq!(&bytes[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
