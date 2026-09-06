use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub fn render_curve(
    points: &[(f64, f64)],
    color: Color,
    stroke: f64,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    // Split on non-finite sentinels (discontinuities) and clamp to a margin
    // around the viewport so far-off-screen coordinates can't produce
    // giant path segments ("random bumps" / streaks across asymptotes).
    let margin_x = (viewport.x2 - viewport.x1).abs() * 0.5 + 1.0;
    let margin_y = (viewport.y2 - viewport.y1).abs() * 0.5 + 1.0;
    let x_lo = viewport.x1.min(viewport.x2) - margin_x;
    let x_hi = viewport.x1.max(viewport.x2) + margin_x;
    let y_lo = viewport.y1.min(viewport.y2) - margin_y;
    let y_hi = viewport.y1.max(viewport.y2) + margin_y;

    let mut seg: Vec<(f64, f64)> = Vec::new();
    let mut flush = |seg: &mut Vec<(f64, f64)>, svg: &mut SvgBuilder| {
        if seg.len() >= 2 {
            let svg_points: Vec<(f64, f64)> =
                seg.iter().map(|(x, y)| viewport.to_svg(*x, *y)).collect();
            svg.path(&svg_points, false, None, color, stroke);
        } else if seg.len() == 1 {
            // Single isolated point: draw a tiny dot-length segment.
            let (x, y) = seg[0];
            let (sx, sy) = viewport.to_svg(x, y);
            svg.path(&[(sx - 0.5, sy), (sx + 0.5, sy)], false, None, color, stroke);
        }
        seg.clear();
    };

    for &(x, y) in points {
        if !x.is_finite() || !y.is_finite() {
            flush(&mut seg, svg);
            continue;
        }
        seg.push((x.clamp(x_lo, x_hi), y.clamp(y_lo, y_hi)));
    }
    flush(&mut seg, svg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_curve() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        let points = vec![(-5.0, 25.0), (0.0, 0.0), (5.0, 25.0)];
        render_curve(&points, Color::rgb(255, 0, 0), 2.0, &vp, &mut svg);
        let result = svg.build();
        assert!(result.contains("<path"));
    }
}
