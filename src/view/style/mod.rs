use tui::style::Style as StyleTuiRs;
use tui::style::Color as ColorTuiRs;

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

            Color::Rgb(r, g, b) => ColorTuiRs::Rgb(*r as u8, *g as u8, *b as u8)
        }
    }
}

pub enum Property {
    Marked,
}

pub struct Style {
    pub color: Color,
    pub property: Option<Property>,
}
// impl Style {
//     fn to_tuirs(&self) -> StyleTuiRs {
//         StyleTuiRs::default().fg(color)
//     }
// }

pub struct Text<'a> {
    pub body: &'a str,
    pub style: Option<Style>,
}

pub struct Texts<'a> {
    pub body: Vec<Text<'a>>,
    // pub body: &'a [Text<'a>],
    // pub stored_vec: Option<Vec<Text<'a>>> // Only necessary when
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

    // pub fn from(value: String) -> Self {
    //     Self::from_str(value.as_str())
    // }
    //
    // pub fn from(value: &String) -> Self {
    //     Self::from_str(value.as_str())
    // }
}

// impl From<&str> for Texts<'_> {
//     fn from(s: &str) -> Self {
//         Self::from_str(s)
//     }
// }

// impl From<String> for Texts<'_> {
//     fn from(value: String) -> Self {
//         Self::from_str(value.as_str())
//     }
// }
//
// impl From<&String> for Texts<'_> {
//     fn from(value: &String) -> Self {
//         Self::from_str(value.as_str())
//     }
// }
