use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub const DEFAULT_POINT_SIZE: f64 = 4.0;

pub fn render_point(
    coords: (f64, f64),
    color: Color,
    size: f64,
    viewport: &Viewport,
    svg: &mut SvgBuilder,
) {
    let (px, py) = viewport.to_svg(coords.0, coords.1);
    svg.circle(px, py, size, Some(color), None);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_point() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_point((0.0, 0.0), Color::rgb(255, 0, 0), 5.0, &vp, &mut svg);
        let result = svg.build();
        assert!(result.contains("<circle"));
    }
}
