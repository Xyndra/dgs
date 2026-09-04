use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub fn render_circle(
    center: (f64, f64),
    radius: f64,
    color: Color,
    stroke: f64,
    fill: Option<Color>,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let (cx, cy) = viewport.to_svg(center.0, center.1);
    let r = radius * viewport.scale_x();
    svg.circle(cx, cy, r, fill, Some((color, stroke)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_circle() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_circle(
            (0.0, 0.0),
            5.0,
            Color::rgb(0, 0, 255),
            1.5,
            Some(Color::new(255, 0, 0, 64)),
            &vp,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<circle"));
    }
}
