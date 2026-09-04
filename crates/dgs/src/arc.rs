use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;
use std::f64::consts::PI;

#[allow(clippy::too_many_arguments)]
pub fn render_arc(
    center: (f64, f64),
    radius: f64,
    start_angle: f64,
    end_angle: f64,
    color: Color,
    stroke: f64,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let start_rad = start_angle * PI / 180.0;
    let end_rad = end_angle * PI / 180.0;

    let sx = viewport.scale_x();
    let r = radius * sx;

    let (ccx, ccy) = viewport.to_svg(center.0, center.1);

    let x1 = ccx + r * start_rad.cos();
    let y1 = ccy - r * start_rad.sin();
    let x2 = ccx + r * end_rad.cos();
    let y2 = ccy - r * end_rad.sin();

    let mut angle_diff = end_angle - start_angle;
    if angle_diff < 0.0 {
        angle_diff += 360.0;
    }
    let large_arc_flag = if angle_diff > 180.0 { 1 } else { 0 };
    let sweep_flag = 1; // counter-clockwise in SVG coords

    let d = format!(
        "M {} {} A {} {} 0 {} {} {} {}",
        fmt(x1),
        fmt(y1),
        fmt(r),
        fmt(r),
        large_arc_flag,
        sweep_flag,
        fmt(x2),
        fmt(y2),
    );

    svg.path_raw(&d, false, None, color, stroke);
}

fn fmt(v: f64) -> String {
    if (v - v.round()).abs() < 1e-10 {
        format!("{}", v as i64)
    } else {
        format!("{:.2}", v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_arc() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_arc(
            (0.0, 0.0),
            5.0,
            0.0,
            90.0,
            Color::rgb(0, 128, 255),
            2.0,
            &vp,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<path"));
        assert!(result.contains("A"));
    }
}
