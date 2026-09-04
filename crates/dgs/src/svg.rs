use crate::color::Color;

pub struct SvgBuilder {
    pub width: f64,
    pub height: f64,
    elements: Vec<String>,
}

impl SvgBuilder {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            elements: Vec::new(),
        }
    }

    pub fn rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fill: Color,
        stroke: Option<(Color, f64)>,
    ) {
        let fill_attr = fill.to_css();
        let stroke_attr = match stroke {
            Some((c, w)) => format!(" stroke=\"{}\" stroke-width=\"{}\"", c.to_css(), w),
            None => String::new(),
        };
        self.elements.push(format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\"{} />",
            fmt(x),
            fmt(y),
            fmt(w),
            fmt(h),
            fill_attr,
            stroke_attr,
        ));
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: Color, width: f64) {
        self.elements.push(format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />",
            fmt(x1),
            fmt(y1),
            fmt(x2),
            fmt(y2),
            stroke.to_css(),
            fmt(width),
        ));
    }

    pub fn circle(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        fill: Option<Color>,
        stroke: Option<(Color, f64)>,
    ) {
        let fill_attr = match fill {
            Some(c) => format!(" fill=\"{}\"", c.to_css()),
            None => " fill=\"none\"".to_string(),
        };
        let stroke_attr = match stroke {
            Some((c, w)) => format!(" stroke=\"{}\" stroke-width=\"{}\"", c.to_css(), fmt(w)),
            None => String::new(),
        };
        self.elements.push(format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"{}{} />",
            fmt(cx),
            fmt(cy),
            fmt(r),
            fill_attr,
            stroke_attr,
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn ellipse(
        &mut self,
        cx: f64,
        cy: f64,
        rx: f64,
        ry: f64,
        rotation: f64,
        fill: Option<Color>,
        stroke: Option<(Color, f64)>,
    ) {
        let fill_attr = match fill {
            Some(c) => format!(" fill=\"{}\"", c.to_css()),
            None => " fill=\"none\"".to_string(),
        };
        let stroke_attr = match stroke {
            Some((c, w)) => format!(" stroke=\"{}\" stroke-width=\"{}\"", c.to_css(), fmt(w)),
            None => String::new(),
        };
        let transform = if rotation != 0.0 {
            format!(
                " transform=\"rotate({} {} {})\"",
                fmt(rotation),
                fmt(cx),
                fmt(cy)
            )
        } else {
            String::new()
        };
        self.elements.push(format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"{}{}{} />",
            fmt(cx),
            fmt(cy),
            fmt(rx),
            fmt(ry),
            fill_attr,
            stroke_attr,
            transform,
        ));
    }

    pub fn path(
        &mut self,
        points: &[(f64, f64)],
        close: bool,
        fill: Option<Color>,
        stroke: Color,
        width: f64,
    ) {
        if points.is_empty() {
            return;
        }
        let mut d = format!("M {} {}", fmt(points[0].0), fmt(points[0].1));
        for &(x, y) in &points[1..] {
            d.push_str(&format!(" L {} {}", fmt(x), fmt(y)));
        }
        if close {
            d.push_str(" Z");
        }
        let fill_attr = match fill {
            Some(c) => format!(" fill=\"{}\"", c.to_css()),
            None => " fill=\"none\"".to_string(),
        };
        self.elements.push(format!(
            "<path d=\"{}\"{} stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\" />",
            d,
            fill_attr,
            stroke.to_css(),
            fmt(width),
        ));
    }

    pub fn path_raw(
        &mut self,
        d: &str,
        close: bool,
        fill: Option<Color>,
        stroke: Color,
        width: f64,
    ) {
        let mut path_d = d.to_string();
        if close {
            path_d.push_str(" Z");
        }
        let fill_attr = match fill {
            Some(c) => format!(" fill=\"{}\"", c.to_css()),
            None => " fill=\"none\"".to_string(),
        };
        self.elements.push(format!(
            "<path d=\"{}\"{} stroke=\"{}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\" />",
            path_d,
            fill_attr,
            stroke.to_css(),
            fmt(width),
        ));
    }

    pub fn text(&mut self, x: f64, y: f64, text: &str, color: Color, size: f64, anchor: &str) {
        self.elements.push(format!(
            "<text x=\"{}\" y=\"{}\" fill=\"{}\" font-size=\"{}\" font-family=\"sans-serif\" text-anchor=\"{}\" dominant-baseline=\"middle\">{}</text>",
            fmt(x),
            fmt(y),
            color.to_css(),
            fmt(size),
            anchor,
            escape_xml(text),
        ));
    }

    pub fn build(self) -> String {
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            fmt(self.width),
            fmt(self.height),
            fmt(self.width),
            fmt(self.height),
        );
        for elem in &self.elements {
            svg.push_str(elem);
            svg.push('\n');
        }
        svg.push_str("</svg>");
        svg
    }
}

fn fmt(v: f64) -> String {
    if (v - v.round()).abs() < 1e-10 {
        format!("{}", v as i64)
    } else {
        format!("{:.2}", v)
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_empty() {
        let svg = SvgBuilder::new(100.0, 100.0).build();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_line() {
        let mut svg = SvgBuilder::new(100.0, 100.0);
        svg.line(0.0, 0.0, 100.0, 100.0, Color::rgb(255, 0, 0), 1.0);
        let result = svg.build();
        assert!(result.contains("<line"));
    }
}
