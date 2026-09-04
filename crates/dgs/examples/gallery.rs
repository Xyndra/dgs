//! Gallery example: builds a scene with loops over outside data and writes
//! SVG + PNG files. Run with `cargo run -p dgs --example gallery`.

use dgs::{color, dgs, eq, line, point, scene};

fn main() {
    let out = std::path::PathBuf::from("/tmp/dgs-gallery");
    std::fs::create_dir_all(&out).unwrap();

    // 1. Builder API with a loop over outside data.
    let data: Vec<(f64, f64)> = (-5..=5).map(|i| (i as f64, (i as f64).sin())).collect();
    let mut s = scene()
        .push(point("O", 0.0, 0.0).with_color(color("red")).with_size(4.0))
        .push(eq("sin(x)").with_range(-5.0, 5.0).with_color(color("blue")));
    for (i, &(x, y)) in data.iter().enumerate() {
        s = s.push(point(format!("P{i}"), x, y).with_size(2.0));
        s = s.push(line("O", format!("P{i}")).with_stroke(0.5));
    }
    std::fs::write(out.join("builder.svg"), s.to_svg()).unwrap();
    #[cfg(feature = "png")]
    std::fs::write(out.join("builder.png"), s.to_png(2.0).unwrap()).unwrap();

    // 2. Macro DSL with loops and conditionals.
    let radii = [1.0, 2.0, 3.0];
    let dark = false;
    let m = dgs! {
        viewport [-5, -5] to [5, 5] size (500, 500);
        if dark { theme dark; } else { theme light; }
        grid spacing 1;
        axes;
        point C (0, 0) size 4 color red;
        for r in radii {
            circle C radius r;
            eq_param "cos(t)" "sin(t)" in [0, 6.28] color green;
        }
    };
    // (radii are unit circles here to keep the demo simple; scale via viewport.)
    std::fs::write(out.join("macro.svg"), m.to_svg()).unwrap();
    #[cfg(feature = "png")]
    std::fs::write(out.join("macro.png"), m.to_png(2.0).unwrap()).unwrap();

    println!("wrote gallery to {}", out.display());
}
