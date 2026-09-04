use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub fn render_grid(
    viewport: &Viewport,
    grid_color: Color,
    grid_width: f64,
    grid_spacing: f64,
    svg: &mut SvgBuilder,
) {
    let x_start = (viewport.x1 / grid_spacing).floor() * grid_spacing;
    let x_end = (viewport.x2 / grid_spacing).ceil() * grid_spacing;
    let y_start = (viewport.y1 / grid_spacing).floor() * grid_spacing;
    let y_end = (viewport.y2 / grid_spacing).ceil() * grid_spacing;

    let mut x = x_start;
    while x <= x_end {
        let (px1, py1) = viewport.to_svg(x, viewport.y1);
        let (px2, py2) = viewport.to_svg(x, viewport.y2);
        svg.line(px1, py1, px2, py2, grid_color, grid_width);
        x += grid_spacing;
    }

    let mut y = y_start;
    while y <= y_end {
        let (px1, py1) = viewport.to_svg(viewport.x1, y);
        let (px2, py2) = viewport.to_svg(viewport.x2, y);
        svg.line(px1, py1, px2, py2, grid_color, grid_width);
        y += grid_spacing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_grid() {
        let vp = Viewport {
            x1: -5.0,
            y1: -5.0,
            x2: 5.0,
            y2: 5.0,
            width: 100.0,
            height: 100.0,
        };
        let mut svg = SvgBuilder::new(100.0, 100.0);
        render_grid(&vp, Color::rgb(200, 200, 200), 0.5, 1.0, &mut svg);
        let result = svg.build();
        assert!(result.contains("<line"));
    }
}
