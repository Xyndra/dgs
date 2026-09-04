use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        color: Option<Color>,
        stroke: Option<f64>,
    },
    #[serde(rename = "curve_param")]
    CurveParam {
        x_expr: String,
        y_expr: String,
        t_min: Option<f64>,
        t_max: Option<f64>,
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

/// Mirrors `dgs-eq(expr)` (`var` defaults to `"x"`, range to `-10..10`).
pub fn eq(expr: &str) -> Object {
    Object::Curve {
        expr_str: expr.to_string(),
        var_name: "x".to_string(),
        t_min: None,
        t_max: None,
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
                color,
                stroke,
            } => {
                let points = evaluate_curve(expr_str, var_name, *t_min, *t_max, lookup);
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
                color,
                stroke,
            } => {
                let points = evaluate_parametric(x_expr, y_expr, *t_min, *t_max);
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

fn evaluate_curve(
    expr_str: &str,
    var_name: &str,
    t_min: Option<f64>,
    t_max: Option<f64>,
    _lookup: &HashMap<String, (f64, f64)>,
) -> Vec<(f64, f64)> {
    let expr = match crate::parser::parse(expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let default_min = -10.0;
    let default_max = 10.0;
    let min = t_min.unwrap_or(default_min);
    let max = t_max.unwrap_or(default_max);
    let steps = 200;
    let dt = (max - min) / steps as f64;

    let mut points = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let t = min + dt * i as f64;
        if let Ok(y) = crate::parser::eval(&expr, &[(var_name, t)]) {
            if y.is_finite() {
                points.push((t, y));
            }
        }
    }

    points
}

fn evaluate_parametric(
    x_expr_str: &str,
    y_expr_str: &str,
    t_min: Option<f64>,
    t_max: Option<f64>,
) -> Vec<(f64, f64)> {
    let x_expr = match crate::parser::parse(x_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let y_expr = match crate::parser::parse(y_expr_str) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let default_min = 0.0;
    let default_max = std::f64::consts::TAU;
    let min = t_min.unwrap_or(default_min);
    let max = t_max.unwrap_or(default_max);
    let steps = 200;
    let dt = (max - min) / steps as f64;

    let mut points = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let t = min + dt * i as f64;
        let x = crate::parser::eval(&x_expr, &[("t", t)]);
        let y = crate::parser::eval(&y_expr, &[("t", t)]);
        if let (Ok(x_val), Ok(y_val)) = (x, y) {
            if x_val.is_finite() && y_val.is_finite() {
                points.push((x_val, y_val));
            }
        }
    }

    points
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
        let points = evaluate_curve("x^2", "x", Some(0.0), Some(2.0), &HashMap::new());
        assert!(!points.is_empty());
        assert!((points[0].1 - 0.0).abs() < 1e-10);
    }
}
