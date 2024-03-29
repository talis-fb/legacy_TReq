use serde::{Deserialize, Serialize};
use tui::style::Color as ColorTuiRs;

pub enum Size {
    Percentage(u16),
    Fixed(u16),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Black,
    White,
    Gray,
    Brown,
    Cyan,
    Pink,
    Magenta,
    Rgb(i32, i32, i32),
}

impl Color {
    pub fn to_tuirs(&self) -> ColorTuiRs {
        match self {
            Color::Red => ColorTuiRs::Red,
            Color::Blue => ColorTuiRs::Blue,
            Color::Green => ColorTuiRs::Green,
            Color::Yellow => ColorTuiRs::Yellow,
            Color::Black => ColorTuiRs::Black,
            Color::White => ColorTuiRs::White,
            Color::Gray => ColorTuiRs::Gray,
            Color::Cyan => ColorTuiRs::Cyan,
            Color::Magenta => ColorTuiRs::Magenta,
            Color::Brown => ColorTuiRs::White,
            Color::Orange => ColorTuiRs::White,
            Color::Pink => ColorTuiRs::White,
            Color::Rgb(r, g, b) => ColorTuiRs::Rgb(*r as u8, *g as u8, *b as u8),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Property {
    Marked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Style {
    pub color: Color,
    pub property: Option<Property>,
}
impl Style {
    pub fn from_color(color: Color) -> Self {
        Self {
            color,
            property: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text<'a> {
    pub body: &'a str,
    pub style: Option<Style>,
}
impl<'a> Text<'a> {
    pub fn from_str(body: &'a str) -> Self {
        Self { body, style: None }
    }

    pub fn from_str_styled(body: &'a str, style: Style) -> Self {
        Self {
            body,
            style: Some(style),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Texts<'a> {
    pub body: Vec<Text<'a>>,
}

impl ToString for Texts<'_> {
    fn to_string(&self) -> String {
        self.body.iter().map(|f| f.body).collect()
    }
}

impl<'a> Texts<'a> {
    pub fn from_str(s: &'a str) -> Self {
        Self {
            body: vec![Text {
                body: s,
                style: None,
            }],
        }
    }

    pub fn from_vec_text(body: Vec<Text<'a>>) -> Self {
        Self { body }
    }
}
