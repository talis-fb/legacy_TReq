use serde::{Deserialize, Serialize};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};


// -----------------------------------------------------------------
// OLD        ------------------------------------------------------
// -----------------------------------------------------------------
#[derive(Deserialize, Serialize, Clone)]
enum StyleOptions {
    ColorCyan,
    ColorRed,
    ColorBlue,
    ColorYellow,
}
impl StyleOptions {
    pub fn to_tui_style(&self) -> Style {
        match self {
            Self::ColorRed => Style::default().fg(Color::LightRed),
            Self::ColorCyan => Style::default().fg(Color::Cyan),
            Self::ColorBlue => Style::default().fg(Color::Blue),
            Self::ColorYellow => Style::default().fg(Color::LightYellow),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DocView {
    content: Vec<Vec<(String, Option<StyleOptions>)>>,
}
impl DocView {
    pub fn from_string(s: String) -> Self {
        let result: Self = serde_json::from_str(&s).unwrap();
        result
    }

    pub fn to_vec_spans(&self) -> Vec<Spans> {
        self.content
            .iter()
            .map(|f| {
                Spans::from(
                    f.iter()
                        .map(|(content, style)| match style {
                            Some(op) => Span::styled(content, op.to_tui_style()),
                            None => Span::from(content.as_str()),
                        })
                        .collect::<Vec<Span>>(),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_json() {
        let data = r#"
        {
            "content": [
                [ ["Some Text", "ColorRed"], ["Some Text", "ColorCyan"], ["Some Text", "ColorCyan"], ["Some Text", null] ],
                [ ["Some another Text", null] ]
            ]
        }"#;

        let parsed: DocView = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn should_return_spans_correct() {
        let data = r#"
        {
            "content": [
                [ ["Text1", "ColorRed"], [ "Text2", null ] ]
            ]
        }"#;

        let parsed: DocView = serde_json::from_str(data).unwrap();

        let spans = parsed.to_vec_spans();
        let wished = vec![Spans::from(vec![
            Span::styled("Text1", Style::default().fg(Color::LightRed)),
            Span::from("Text2"),
        ])];

        assert_eq!(spans, wished);
    }

    #[test]
    fn should_return_spans_correct_with_multiple_lines() {
        let data = r#"
        {
            "content": [
                [ ["Text1", "ColorRed"] ],
                [ ["Text2", null], ["Text3", "ColorYellow"] ],
                [ ["Text4", null] ]
            ]
        }"#;

        let parsed: DocView = serde_json::from_str(data).unwrap();

        let spans = parsed.to_vec_spans();
        let wished = vec![
            Spans::from(vec![Span::styled(
                "Text1",
                Style::default().fg(Color::LightRed),
            )]),
            Spans::from(vec![
                Span::from("Text2"),
                Span::styled("Text3", Style::default().fg(Color::LightYellow)),
            ]),
            Spans::from(vec![Span::from("Text4")]),
        ];

        assert_eq!(spans, wished);
    }
}
