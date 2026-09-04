//! # dgs — Dynamic Geometry Software for Rust
//!
//! Describe geometric constructions — points, lines, circles, polygons,
//! ellipses, arcs, and expression-driven curves — in math coordinates and
//! render them to SVG or PNG.
//!
//! The API mirrors the [typst_dgs](https://github.com/xyndra/typst_dgs) Typst
//! functions (`dgs-point`, `dgs-line`, `dgs-circle`, …, `dgs-canvas`), minus
//! the `dgs-` prefix:
//!
//! ```
//! use dgs::{circle, color, line, point, scene};
//!
//! let s = scene()
//!     .push(point("A", 0.0, 0.0).with_color(color("red")))
//!     .push(point("B", 3.0, 4.0))
//!     .push(line("A", "B"))
//!     .push(circle("A", 2.0).with_fill(color("#0000ff40")));
//! let svg = s.to_svg();
//! assert!(svg.starts_with("<svg"));
//! ```
//!
//! Or use the [`dgs!`] DSL macro, which also supports Rust control flow over
//! outside data:
//!
//! ```
//! use dgs::dgs;
//!
//! let pts = vec![(0.0, 0.0), (3.0, 4.0), (3.0, 0.0)];
//! let scene = dgs! {
//!     viewport [-5, -5] to [5, 5] size (400, 400);
//!     theme light;
//!     grid spacing 1;
//!     axes;
//!     point A (0, 0);
//!     for (x, y) in pts {
//!         point (x, y) color blue;
//!     }
//!     polygon A (3, 4) (3, 0) fill "#0000ff40";
//!     eq "sin(x) / x" in [-5, 5] color "#ff6600";
//!     eq_param "2 * cos(t)" "2 * sin(t)" in [0, 6.28];
//! };
//! assert!(scene.to_svg().starts_with("<svg"));
//! ```
//!
//! Curve expressions use the [`parser`] math DSL, e.g. `"sin(x) / x"`,
//! with constants `pi`/`e` and the usual functions (`sin`, `cos`, `sqrt`, …).

pub mod arc;
pub mod axis;
pub mod circle;
pub mod color;
pub mod curve;
pub mod ellipse;
pub mod grid;
pub mod line;
pub mod objects;
pub mod parser;
pub mod point;
pub mod polygon;
pub mod scene;
pub mod semicircle;
pub mod svg;
pub mod theme;
pub mod viewport;

pub use color::Color;
pub use objects::{
    arc, circle, color, ellipse, eq, eq_param, line, point, point_at, polygon, semicircle,
    Object, PointRef, SemicircleDir,
};
pub use scene::{scene, Format, RenderError, Renderer, Scene, SvgRenderer};
#[cfg(feature = "png")]
pub use scene::PngRenderer;
pub use theme::Theme;
pub use viewport::Viewport;

pub use parser::{eval, eval_str, parse, Expr};

/// The `dgs!` DSL macro, re-exported so `dgs` is the only crate you depend on.
pub use dgs_macros::dgs;

/// Everything the [`dgs!`] macro expansion refers to. Not a stable API.
#[doc(hidden)]
pub mod __private {
    pub use crate::objects::{
        arc, circle, color, ellipse, eq, eq_param, line, point, point_at, polygon,
        semicircle, Object, PointRef, SemicircleDir,
    };
    pub use crate::scene::Scene;
    pub use crate::theme::Theme;
    pub use crate::viewport::Viewport;
    pub use crate::Color;
}
