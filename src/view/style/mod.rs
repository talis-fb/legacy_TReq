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
    Rgb(i32, i32, i32),
}

pub enum Property {
    Marked,
}

pub struct Style {
    pub color: Option<Color>,
    pub property: Option<Property>,
}

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
