use crate::color::Color;
use crate::svg::SvgBuilder;
use crate::viewport::Viewport;

pub fn render_axes(
    viewport: &Viewport,
    axis_color: Color,
    axis_width: f64,
    label_color: Color,
    label_size: f64,
    svg: &mut SvgBuilder,
) {
    // X-axis (y=0)
    if viewport.y1 <= 0.0 && viewport.y2 >= 0.0 {
        let (x1, y1) = viewport.to_svg(viewport.x1, 0.0);
        let (x2, y2) = viewport.to_svg(viewport.x2, 0.0);
        svg.line(x1, y1, x2, y2, axis_color, axis_width);
    }

    // Y-axis (x=0)
    if viewport.x1 <= 0.0 && viewport.x2 >= 0.0 {
        let (x1, y1) = viewport.to_svg(0.0, viewport.y1);
        let (x2, y2) = viewport.to_svg(0.0, viewport.y2);
        svg.line(x1, y1, x2, y2, axis_color, axis_width);
    }

    // Tick marks and labels on x-axis
    if viewport.y1 <= 0.0 && viewport.y2 >= 0.0 {
        let (_, axis_y) = viewport.to_svg(0.0, 0.0);
        let tick_len = 5.0;
        let x_start = viewport.x1.ceil() as i64;
        let x_end = viewport.x2.floor() as i64;
        for x in x_start..=x_end {
            if x == 0 {
                continue;
            }
            let (px, _) = viewport.to_svg(x as f64, 0.0);
            svg.line(
                px,
                axis_y - tick_len,
                px,
                axis_y + tick_len,
                axis_color,
                axis_width,
            );
            svg.text(
                px,
                axis_y + tick_len + label_size,
                &x.to_string(),
                label_color,
                label_size,
                "middle",
            );
        }
        // "x" label at right end
        let (lx, _) = viewport.to_svg(viewport.x2, 0.0);
        svg.text(
            lx - label_size,
            axis_y - tick_len - 2.0,
            "x",
            label_color,
            label_size,
            "end",
        );
    }

    // Tick marks and labels on y-axis
    if viewport.x1 <= 0.0 && viewport.x2 >= 0.0 {
        let (axis_x, _) = viewport.to_svg(0.0, 0.0);
        let tick_len = 5.0;
        let y_start = viewport.y1.ceil() as i64;
        let y_end = viewport.y2.floor() as i64;
        for y in y_start..=y_end {
            if y == 0 {
                continue;
            }
            let (_, py) = viewport.to_svg(0.0, y as f64);
            svg.line(
                axis_x - tick_len,
                py,
                axis_x + tick_len,
                py,
                axis_color,
                axis_width,
            );
            svg.text(
                axis_x - tick_len - 4.0,
                py,
                &y.to_string(),
                label_color,
                label_size,
                "end",
            );
        }
        // "y" label at top
        let (_, ly) = viewport.to_svg(0.0, viewport.y2);
        svg.text(
            axis_x + tick_len + 4.0,
            ly + label_size,
            "y",
            label_color,
            label_size,
            "start",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_axes() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let mut svg = SvgBuilder::new(200.0, 200.0);
        render_axes(
            &vp,
            Color::rgb(0, 0, 0),
            1.5,
            Color::rgb(100, 100, 100),
            10.0,
            &mut svg,
        );
        let result = svg.build();
        assert!(result.contains("<line"));
        assert!(result.contains("<text"));
    }
}
