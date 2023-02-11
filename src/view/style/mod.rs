
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
    Rgb(i32, i32, i32)
}

pub enum Property {
    Marked
}

pub struct  Style {
    pub color: Option<Color>,
    pub property: Option<Property>,
}

pub struct Text<'a> {
    pub body: &'a str,
    pub style: Option<Style>
}

pub type Texts<'a> = &'a [Text<'a>];
