use crate::view::style::{Color, Style, Text, Texts};
use serde::{Deserialize, Serialize};
use tui::{
    style::{Color as ColorTuiRs, Style as StyleTuiRs},
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
    pub fn to_tui_style(&self) -> StyleTuiRs {
        match self {
            Self::ColorRed => StyleTuiRs::default().fg(ColorTuiRs::LightRed),
            Self::ColorCyan => StyleTuiRs::default().fg(ColorTuiRs::Cyan),
            Self::ColorBlue => StyleTuiRs::default().fg(ColorTuiRs::Blue),
            Self::ColorYellow => StyleTuiRs::default().fg(ColorTuiRs::LightYellow),
        }
    }

    pub fn to_style(&self) -> Style {
        match self {
            Self::ColorRed => Style {
                color: Color::Red,
                property: None,
            },
            Self::ColorCyan => Style {
                color: Color::Cyan,
                property: None,
            },
            Self::ColorBlue => Style {
                color: Color::Blue,
                property: None,
            },
            Self::ColorYellow => Style {
                color: Color::Yellow,
                property: None,
            },
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
    pub fn to_texts_spans(&self) -> Vec<Texts> {
        self.content
            .iter()
            .map(|f| Texts {
                body: f
                    .iter()
                    .map(|(content, style)| match style {
                        Some(op) => Text {
                            body: &content,
                            style: Some(op.to_style()),
                        },
                        None => Text {
                            body: &content,
                            style: None,
                        },
                    })
                    .collect::<Vec<Text>>(),
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

    mod parse_tui_rs {
        use super::*;

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
                Span::styled("Text1", StyleTuiRs::default().fg(ColorTuiRs::LightRed)),
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
                    StyleTuiRs::default().fg(ColorTuiRs::LightRed),
                )]),
                Spans::from(vec![
                    Span::from("Text2"),
                    Span::styled("Text3", StyleTuiRs::default().fg(ColorTuiRs::LightYellow)),
                ]),
                Spans::from(vec![Span::from("Text4")]),
            ];

            assert_eq!(spans, wished);
        }
    }

    mod parse_texts {

        use super::*;

        #[test]
        fn should_return_spans_correct() {
            let data = r#"
        {
            "content": [
                [ ["Text1", "ColorRed"], [ "Text2", null ] ]
            ]
        }"#;

            let parsed: DocView = serde_json::from_str(data).unwrap();

            let spans = parsed.to_texts_spans();
            let wished = vec![Texts {
                body: vec![
                    Text {
                        body: "Text1",
                        style: Some(Style {
                            color: Color::Red,
                            property: None,
                        }),
                    },
                    Text {
                        body: "Text2",
                        style: None,
                    },
                ],
            }];

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

            let spans = parsed.to_texts_spans();
            let wished = vec![
                Texts {
                    body: vec![Text {
                        body: "Text1",
                        style: Some(Style {
                            color: Color::Red,
                            property: None,
                        }),
                    }],
                },
                Texts {
                    body: vec![
                        Text {
                            body: "Text2",
                            style: None,
                        },
                        Text {
                            body: "Text3",
                            style: Some(Style {
                                color: Color::Yellow,
                                property: None,
                            }),
                        },
                    ],
                },
                Texts {
                    body: vec![Text {
                        body: "Text4",
                        style: None,
                    }],
                },
            ];

            assert_eq!(spans, wished);
        }
    }
}
