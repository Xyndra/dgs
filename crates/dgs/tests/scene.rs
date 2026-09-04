//! Integration tests for the Typst-like free-function + `Scene` API.

use dgs::{
    arc, circle, color, ellipse, eq, eq_param, line, point, point_at, polygon, scene,
    semicircle, Format, SemicircleDir,
};

#[test]
fn typst_like_api() {
    let s = scene()
        .push(point("A", 0.0, 0.0).with_color(color("red")).with_size(3.0))
        .push(point("B", 3.0, 4.0))
        .push(point_at(1.0, 1.0))
        .push(line("A", "B").with_stroke(2.0))
        .push(line("A", (5.0, 5.0)))
        .push(circle("A", 2.0).with_fill(color("#0000ff40")))
        .push(polygon(vec!["A".into(), "B".into(), (3.0, 0.0).into()]))
        .push(ellipse("A", 3.0, 2.0).with_rotation(30.0))
        .push(arc("A", 2.0, 0.0, 90.0))
        .push(semicircle("A", "B").with_dir(SemicircleDir::Clockwise))
        .push(semicircle("A", "B").with_dir("cw".parse().unwrap()))
        .push(eq("sin(x) / x").with_range(-5.0, 5.0))
        .push(eq("x^2").with_var("x").with_color(color("green")))
        .push(eq_param("2 * cos(t)", "2 * sin(t)").with_range(0.0, 6.28));
    let svg = s.to_svg();
    assert!(svg.starts_with("<svg"));
}

#[test]
fn dynamic_from_iterator() {
    let pts = vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)];
    let s = scene().extend(pts.iter().map(|&(x, y)| point_at(x, y)));
    assert!(s.to_svg().starts_with("<svg"));
}

#[test]
fn render_format() {
    let bytes = scene().render(Format::Svg).unwrap();
    assert!(bytes.starts_with(b"<svg"));
}

#[cfg(feature = "png")]
#[test]
fn render_png_bytes() {
    let bytes = scene().render(Format::Png { scale: 1.0 }).unwrap();
    assert_eq!(&bytes[..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
}
