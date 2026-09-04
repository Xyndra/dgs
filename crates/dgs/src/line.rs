use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub const DEFAULT_STROKE: f64 = 1.5;

pub fn render_line(
    from: (f64, f64),
    to: (f64, f64),
    color: Color,
    stroke: f64,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let (x1, y1) = viewport.to_svg(from.0, from.1);
    let (x2, y2) = viewport.to_svg(to.0, to.1);
    svg.line(x1, y1, x2, y2, color, stroke);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_line() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_line(
            (-5.0, -5.0),
            (5.0, 5.0),
            Color::rgb(0, 0, 255),
            2.0,
            &vp,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<line"));
    }
}
