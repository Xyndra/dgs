use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, width: f64, height: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            width,
            height,
        }
    }

    pub fn to_svg(&self, x: f64, y: f64) -> (f64, f64) {
        let px = (x - self.x1) / (self.x2 - self.x1) * self.width;
        let py = self.height - (y - self.y1) / (self.y2 - self.y1) * self.height;
        (px, py)
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x1 && x <= self.x2 && y >= self.y1 && y <= self.y2
    }

    pub fn scale_x(&self) -> f64 {
        self.width / (self.x2 - self.x1)
    }

    pub fn scale_y(&self) -> f64 {
        self.height / (self.y2 - self.y1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_svg_origin() {
        let vp = Viewport {
            x1: -10.0,
            y1: -10.0,
            x2: 10.0,
            y2: 10.0,
            width: 200.0,
            height: 200.0,
        };
        let (px, py) = vp.to_svg(0.0, 0.0);
        assert!((px - 100.0).abs() < 1e-10);
        assert!((py - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_contains() {
        let vp = Viewport {
            x1: 0.0,
            y1: 0.0,
            x2: 10.0,
            y2: 10.0,
            width: 100.0,
            height: 100.0,
        };
        assert!(vp.contains(5.0, 5.0));
        assert!(!vp.contains(-1.0, 5.0));
    }
}
