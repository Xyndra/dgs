use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub fn render_polygon(
    points: &[(f64, f64)],
    color: Color,
    stroke: f64,
    fill: Option<Color>,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let svg_points: Vec<(f64, f64)> = points
        .iter()
        .map(|(x, y)| viewport.to_svg(*x, *y))
        .collect();
    svg.path(&svg_points, true, fill, color, stroke);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_polygon() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        let points = vec![(0.0, 5.0), (-5.0, -5.0), (5.0, -5.0)];
        render_polygon(
            &points,
            Color::rgb(0, 0, 255),
            2.0,
            Some(Color::new(0, 255, 0, 64)),
            &vp,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<path"));
        assert!(result.contains("Z"));
    }
}
