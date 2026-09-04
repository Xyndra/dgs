use crate::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Theme {
    Named(String),
    Custom {
        background: Color,
        grid_color: Color,
        axis_color: Color,
        axis_label_color: Color,
        text_color: Color,
    },
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Named("light".to_string())
    }
}

impl Theme {
    pub fn light() -> Self {
        Theme::Named("light".to_string())
    }

    pub fn dark() -> Self {
        Theme::Named("dark".to_string())
    }

    pub fn custom(
        background: Color,
        grid_color: Color,
        axis_color: Color,
        axis_label_color: Color,
        text_color: Color,
    ) -> Self {
        Theme::Custom {
            background,
            grid_color,
            axis_color,
            axis_label_color,
            text_color,
        }
    }

    pub fn background(&self) -> Color {
        match self {
            Theme::Named(s) => match s.as_str() {
                "dark" => Color::rgb(30, 30, 46),
                _ => Color::rgb(255, 255, 255), // light
            },
            Theme::Custom { background, .. } => *background,
        }
    }

    pub fn grid_color(&self) -> Color {
        match self {
            Theme::Named(s) => match s.as_str() {
                "dark" => Color::rgb(64, 64, 80),
                _ => Color::rgb(224, 224, 224),
            },
            Theme::Custom { grid_color, .. } => *grid_color,
        }
    }

    pub fn axis_color(&self) -> Color {
        match self {
            Theme::Named(s) => match s.as_str() {
                "dark" => Color::rgb(192, 192, 192),
                _ => Color::rgb(0, 0, 0),
            },
            Theme::Custom { axis_color, .. } => *axis_color,
        }
    }

    pub fn axis_label_color(&self) -> Color {
        match self {
            Theme::Named(s) => match s.as_str() {
                "dark" => Color::rgb(192, 192, 192),
                _ => Color::rgb(0, 0, 0),
            },
            Theme::Custom {
                axis_label_color, ..
            } => *axis_label_color,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            Theme::Named(s) => match s.as_str() {
                "dark" => Color::rgb(255, 255, 255),
                _ => Color::rgb(0, 0, 0),
            },
            Theme::Custom { text_color, .. } => *text_color,
        }
    }
}
