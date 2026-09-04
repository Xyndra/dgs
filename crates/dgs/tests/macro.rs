//! Integration tests for the `dgs!` macro (evaluates to a `Scene`).

use dgs::{dgs, Scene};

#[test]
fn macro_full_scene() {
    let scene: Scene = dgs! {
        viewport [-5, -5] to [5, 5] size (400, 400);
        theme light;
        grid spacing 1;
        axes;
        point A (0, 0);
        point B (3, 4) color red;
        line A -> B;
        circle A radius 2 stroke 1.5;
        polygon A B (3, 0) fill "#0000ff40";
        ellipse (0, 0) rx 3 ry 2 rot 30 color blue;
        arc A radius 2 from 0 to 90;
        semicircle A -> B cw;
        eq "sin(x) / x" x in [-5, 5] color "#ff6600";
        eq_param "2 * cos(t)" "2 * sin(t)" in [0, 6.28];
        point (1, 2) size 3 color green;
    };
    let svg = scene.to_svg();
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("circle"));
}

#[test]
fn macro_loops_over_outside_data() {
    let pts = vec![(0.0, 0.0), (1.0, 2.0), (-2.0, 1.0)];
    let show_circle = true;

    let scene: Scene = dgs! {
        point A (0, 0);
        for &(x, y) in &pts {
            point (x, y) color blue;
            circle (x, y) radius 0.2;
        }
        #let mut i = 0;
        while i < 3 {
            point (i as f64, 0) size 2;
            #i += 1;
        }
        if show_circle {
            circle A radius 3 color red;
        } else {
            circle A radius 1 color green;
        }
    };
    let svg = scene.to_svg();
    assert!(svg.starts_with("<svg"));
    // 1 named + 3 loop + 3 while = 7 point circles + 4 explicit circles
    assert!(svg.matches("<circle").count() >= 11);
}

#[test]
fn macro_dark_theme_no_viewport() {
    let scene: Scene = dgs! {
        theme dark;
        grid;
        axes;
        point (0, 0);
    };
    assert!(scene.to_svg().starts_with("<svg"));
}

#[test]
fn macro_curve_aliases() {
    let scene: Scene = dgs! {
        curve "x^2" x in [-2, 2];
        curve_param "cos(t)" "sin(t)" in [0, 6.28];
    };
    assert!(scene.to_svg().starts_with("<svg"));
}
