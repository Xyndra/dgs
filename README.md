# dgs

Dynamic geometry software (GeoGebra-like canvas) for Rust: describe points,
lines, circles, polygons, ellipses, arcs, and expression-driven curves in math
coordinates and render them to SVG or PNG.

The API mirrors the [typst_dgs](https://github.com/xyndra/typst_dgs) Typst
functions (`dgs-point`, `dgs-line`, `dgs-circle`, …, `dgs-canvas`), minus the
`dgs-` prefix. `dgs` is the only crate you need to depend on — it re-exports
everything, including the `dgs!` macro.

## Builder / free-function API

```rust
use dgs::{circle, color, line, point, scene};

let s = scene()
    .push(point("A", 0.0, 0.0).with_color(color("red")))
    .push(point("B", 3.0, 4.0))
    .push(line("A", "B"))
    .push(circle("A", 2.0).with_fill(color("#0000ff40")));

let svg: String = s.to_svg();
let png: Vec<u8> = s.to_png(2.0).unwrap(); // `png` feature (default)
```

## `dgs!` DSL macro

```rust
use dgs::dgs;

let pts = vec![(1.0, 1.0), (2.0, 3.0)];
let scene = dgs! {
    viewport [-5, -5] to [5, 5] size (400, 400);
    theme light;
    grid spacing 1;
    axes;
    point A (0, 0);
    for (x, y) in pts {          // Rust control flow over outside data
        point (x, y) color blue;
    }
    if pts.len() > 1 {
        polygon A (1, 1) (2, 3);
    }
    eq "sin(x) / x" in [-5, 5] color "#ff6600";       // `eq` = Typst's dgs-eq
    eq_param "2*cos(t)" "2*sin(t)" in [0, 6.28];
};
```

The macro evaluates to a [`Scene`](https://docs.rs/dgs), so you can keep
building with Rust code afterwards and render to SVG or PNG.

Curve expressions use a small built-in math DSL (`sin`, `cos`, `sqrt`, `pi`,
…) and are sampled at render time.

## Rendering

`Scene::to_svg()`, `Scene::to_png(scale)` (needs the default `png` feature),
`Scene::render(Format::Svg | Format::Png { scale })`, or implement the
`Renderer` trait for future formats. Disable default features for lean/no-std-ish
targets such as WASM plugins:

```toml
dgs = { version = "0.1", default-features = false }
```

## Crates in this repo

| Crate | Published | Purpose |
|---|---|---|
| `dgs` | yes | The single public crate: scene, API, renderers, re-exported macro |
| `dgs-macros` | yes (impl detail) | Proc-macro behind `dgs!`; use via `dgs::dgs` |

License: MIT OR Apache-2.0
