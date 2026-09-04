use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn parse(s: &str) -> Option<Self> {
        parse_color(s)
    }

    pub fn to_css(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!(
                "rgba({},{},{},{})",
                self.r,
                self.g,
                self.b,
                self.a as f64 / 255.0
            )
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(0, 0, 0)
    }
}

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Color::parse(s).ok_or_else(|| format!("invalid color: {s}"))
    }
}

// --- Serialize as hex string ---

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_css())
    }
}

// --- Deserialize from string (hex or named color) ---

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a color string like \"red\", \"#ff0000\", or \"#ff000080\"")
    }

    fn visit_str<E>(self, value: &str) -> Result<Color, E>
    where
        E: de::Error,
    {
        // Handle Typst repr() format: rgb("#ff0000"), luma(66.67%), etc.
        let cleaned = value
            .trim_start_matches("rgb(")
            .trim_start_matches("rgba(")
            .trim_start_matches("luma(")
            .trim_start_matches("Color::Rgb(")
            .trim_end_matches(')')
            .trim_matches('"')
            .trim();

        // Handle luma percentage format: "66.67%" -> convert to gray value
        if cleaned.ends_with('%') {
            if let Some(pct_str) = cleaned.strip_suffix('%') {
                if let Ok(pct) = pct_str.trim().parse::<f64>() {
                    let v = (pct / 100.0 * 255.0) as u8;
                    return Ok(Color::rgb(v, v, v));
                }
            }
        }

        parse_color(cleaned)
            .or_else(|| parse_color(value))
            .ok_or_else(|| E::custom(format!("invalid color: {}", value)))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

// --- Hex parsing helpers ---

fn hex_digit(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

fn parse_hex_component(s: &str) -> Option<u8> {
    match s.len() {
        1 => {
            let hi = hex_digit(s.chars().next()?)?;
            Some(hi * 17)
        }
        2 => {
            let mut chars = s.chars();
            let hi = hex_digit(chars.next()?)?;
            let lo = hex_digit(chars.next()?)?;
            Some(hi * 16 + lo)
        }
        _ => None,
    }
}

pub fn parse_color(s: &str) -> Option<Color> {
    let s = s.trim();
    match s.to_lowercase().as_str() {
        "black" => return Some(Color::rgb(0, 0, 0)),
        "white" => return Some(Color::rgb(255, 255, 255)),
        "red" => return Some(Color::rgb(255, 0, 0)),
        "green" => return Some(Color::rgb(0, 128, 0)),
        "blue" => return Some(Color::rgb(0, 0, 255)),
        "yellow" => return Some(Color::rgb(255, 255, 0)),
        "cyan" => return Some(Color::rgb(0, 255, 255)),
        "magenta" => return Some(Color::rgb(255, 0, 255)),
        "orange" => return Some(Color::rgb(255, 165, 0)),
        "purple" => return Some(Color::rgb(128, 0, 128)),
        "gray" | "grey" => return Some(Color::rgb(128, 128, 128)),
        "darkgray" | "darkgrey" => return Some(Color::rgb(64, 64, 64)),
        "lightgray" | "lightgrey" => return Some(Color::rgb(192, 192, 192)),
        _ => {}
    }

    let hex = s.strip_prefix('#')?;

    match hex.len() {
        3 => {
            let r = parse_hex_component(&hex[0..1])?;
            let g = parse_hex_component(&hex[1..2])?;
            let b = parse_hex_component(&hex[2..3])?;
            Some(Color::rgb(r, g, b))
        }
        4 => {
            let r = parse_hex_component(&hex[0..1])?;
            let g = parse_hex_component(&hex[1..2])?;
            let b = parse_hex_component(&hex[2..3])?;
            let a = parse_hex_component(&hex[3..4])?;
            Some(Color::new(r, g, b, a))
        }
        6 => {
            let r = parse_hex_component(&hex[0..2])?;
            let g = parse_hex_component(&hex[2..4])?;
            let b = parse_hex_component(&hex[4..6])?;
            Some(Color::rgb(r, g, b))
        }
        8 => {
            let r = parse_hex_component(&hex[0..2])?;
            let g = parse_hex_component(&hex[2..4])?;
            let b = parse_hex_component(&hex[4..6])?;
            let a = parse_hex_component(&hex[6..8])?;
            Some(Color::new(r, g, b, a))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_colors() {
        assert_eq!(parse_color("red"), Some(Color::rgb(255, 0, 0)));
        assert_eq!(parse_color("BLUE"), Some(Color::rgb(0, 0, 255)));
    }

    #[test]
    fn test_hex_6() {
        assert_eq!(parse_color("#ff8800"), Some(Color::rgb(255, 136, 0)));
    }

    #[test]
    fn test_hex_8() {
        assert_eq!(parse_color("#ff000080"), Some(Color::new(255, 0, 0, 128)));
    }

    #[test]
    fn test_hex_3() {
        assert_eq!(parse_color("#f00"), Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_to_css_opaque() {
        let c = Color::rgb(255, 128, 0);
        assert_eq!(c.to_css(), "#ff8000");
    }

    #[test]
    fn test_to_css_alpha() {
        let c = Color::new(255, 0, 0, 128);
        assert!(c.to_css().contains("rgba"));
    }

    #[test]
    fn test_serialize() {
        let c = Color::rgb(255, 0, 0);
        // Just verify to_css works (the actual serialization depends on the format)
        assert_eq!(c.to_css(), "#ff0000");
    }
}
