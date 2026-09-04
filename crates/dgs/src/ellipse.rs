use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

#[allow(clippy::too_many_arguments)]
pub fn render_ellipse(
    center: (f64, f64),
    rx: f64,
    ry: f64,
    rotation: f64,
    color: Color,
    stroke: f64,
    fill: Option<Color>,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let (cx, cy) = viewport.to_svg(center.0, center.1);
    let sx = viewport.scale_x();
    let sy = viewport.scale_y();
    svg.ellipse(
        cx,
        cy,
        rx * sx,
        ry * sy,
        rotation,
        fill,
        Some((color, stroke)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_ellipse() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_ellipse(
            (0.0, 0.0),
            5.0,
            3.0,
            45.0,
            Color::rgb(255, 0, 255),
            1.5,
            None,
            &vp,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<ellipse"));
    }
}
