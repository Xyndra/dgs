use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;
use std::f64::consts::PI;

pub fn render_semicircle(
    from: (f64, f64),
    to: (f64, f64),
    center: Option<(f64, f64)>,
    dir: &str,
    color: Color,
    stroke: f64,
    fill: Option<Color>,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let (cx, cy, r) = if let Some(c) = center {
        let r = ((from.0 - c.0).powi(2) + (from.1 - c.1).powi(2)).sqrt();
        (c.0, c.1, r)
    } else {
        let cx = (from.0 + to.0) / 2.0;
        let cy = (from.1 + to.1) / 2.0;
        let r = ((to.0 - from.0).powi(2) + (to.1 - from.1).powi(2)).sqrt() / 2.0;
        (cx, cy, r)
    };

    let ang_a = (from.1 - cy).atan2(from.0 - cx) * 180.0 / PI;
    let ang_b = (to.1 - cy).atan2(to.0 - cx) * 180.0 / PI;

    let is_cw = dir.to_lowercase().starts_with("cw") || dir.to_lowercase() == "clockwise";

    let diff = if is_cw {
        let mut d = ang_a - ang_b;
        if d < 0.0 {
            d += 360.0;
        }
        d
    } else {
        let mut d = ang_b - ang_a;
        if d < 0.0 {
            d += 360.0;
        }
        d
    };

    let s_ang = ang_a;
    let e_ang = ang_b;

    let large = if diff > 180.0 { 1 } else { 0 };
    let sweep = if is_cw { 1 } else { 0 };

    let sx = viewport.scale_x();
    let sr = r * sx;
    let (ccx, ccy) = viewport.to_svg(cx, cy);
    let s_rad = s_ang * PI / 180.0;
    let e_rad = e_ang * PI / 180.0;
    let x1 = ccx + sr * s_rad.cos();
    let y1 = ccy - sr * s_rad.sin();
    let x2 = ccx + sr * e_rad.cos();
    let y2 = ccy - sr * e_rad.sin();

    if let Some(fill_color) = fill {
        let d_fill = format!(
            "M {} {} L {} {} A {} {} 0 {} {} {} {} Z",
            fmt(ccx),
            fmt(ccy),
            fmt(x1),
            fmt(y1),
            fmt(sr),
            fmt(sr),
            large,
            sweep,
            fmt(x2),
            fmt(y2)
        );
        svg.path_raw(&d_fill, true, Some(fill_color), color, stroke);
    } else {
        let d = format!(
            "M {} {} A {} {} 0 {} {} {} {}",
            fmt(x1),
            fmt(y1),
            fmt(sr),
            fmt(sr),
            large,
            sweep,
            fmt(x2),
            fmt(y2)
        );
        svg.path_raw(&d, false, None, color, stroke);
    }
}

fn fmt(v: f64) -> String {
    if (v - v.round()).abs() < 1e-10 {
        format!("{}", v as i64)
    } else {
        format!("{:.2}", v)
    }
}
