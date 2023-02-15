use super::Component;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Style, Texts};
use crate::view::{renderer::tui_rs::BackendTuiRs, style::Text};
use tui::layout::{Constraint, Layout, Rect};

pub struct WelcomeDoc {
    pub area: Rect,
    pub marked: bool,
}
impl Component for WelcomeDoc {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        // Has height 11
        let title = r#" ⠀⠀⠀⠀⠀⠀⠀⣀⣀⣠⣤⣀⣀⡀                              ⠀
⠀⠀⠀⢀⣤⠶⠛⠉⠉⠁⠀⠉⠉⠛⠳⣦⣀⠀⠀⠀⠀  ___________           
⠀⠀⣴⠟⠁⠀⠀⠀⠀⢀⣤⣶⣤⡀⠀⠀⠙⢷⡀⠀⠀ |_   _| ___ \          
⠀⣾⠃⠀⠀⠀⠀⣐⣾⣿⣿⣿⢆⣯⠳⣦⣤⣈⢻⡄⠀   | | | |_/ /___  __ _ 
⣸⠇⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⡟⠛⠶⣷⣿⣿⡇⣿⠀   | | |    // _ \/ _` |
⣿⠀⠀⠀⠀⢀⣸⣿⣿⡿⢿⣿⣿⣄⠈⡉⠉⠉⠀⢸⡆   | | | |\ \  __/ (_| |
⢿⣀⣠⣴⣷⣿⣿⡟⠉⢠⠆⠉⠉⠻⠿⠿⠀⠀⠀⣼⠃   \_/ \_| \_\___|\__, |
⠸⣿⣿⣿⣿⡿⣿⣷⣶⡺⠞⠃⠀⠀⠀⠀⠀⠀⢠⡟⠀                     | |
⠀⠹⣿⣿⢟⠗⠀⠈⠉⠁⠀⠀⠀⢀⠀⣀⠀⣠⡟⠁⠀                     |_|
⠀⠀⠈⠳⣤⡀⣀⣰⣶⣶⣴⣷⣶⣿⣷⣿⡿⠋⠀⠀⠀                        
⠀⠀⠀⠀⠈⠙⠻⢿⣿⣿⣿⣿⡿⠿⠛⠉⠀⠀⠀⠀⠀                        
"#;

        let rows = vec![
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("For get information of usage press the key "),
                Text::from_str_styled("[?]", Style::from_color(Color::Blue)),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("For MORE information:"),
                Text::from_str_styled(
                    "https://github.com/talis-fb/TReq/wiki",
                    Style::from_color(Color::Red),
                ),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![Text::from_str_styled(
                "-- Little Tour --",
                Style::from_color(Color::Yellow),
            )]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("Use "),
                Text::from_str_styled("[ARROW KEYS]", Style::from_color(Color::Blue)),
                Text::from_str(" or "),
                Text::from_str_styled("[h/j/k/l]", Style::from_color(Color::Blue)),
                Text::from_str(" to navegate between sections of app"),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("Focus on URL section and press the key "),
                Text::from_str_styled("[e]", Style::from_color(Color::Blue)),
                Text::from_str(", a little pop up to set up text will appear. Set the URL and press "),
                Text::from_str_styled("[ENTER]", Style::from_color(Color::Blue)),
                Text::from_str(" to confirm."),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("Still in URL Section, you can press "),
                Text::from_str_styled("[TAB]", Style::from_color(Color::Blue)),
                Text::from_str(" to change Method to use in Request"),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("Press "),
                Text::from_str_styled("[ENTER]", Style::from_color(Color::Blue)),
                Text::from_str(" in any section to submit the Request and see the response HERE"),
            ]),
            Texts::from_str(""),
            Texts::from_vec_text(vec![
                Text::from_str("To quit, you can press "),
                Text::from_str_styled("[q]", Style::from_color(Color::Blue)),
            ]),
        ];

        f.render_clear_area(self.area);

        if self.marked {
            f.render_block_with_title_center_marked(Texts::from_str("Welcome"), self.area);
        } else {
            f.render_block_with_title_center(Texts::from_str("Welcome"), self.area);
        }

        let content_area = Layout::default()
            .margin(1)
            .constraints([Constraint::Length(11), Constraint::Min(1)])
            .split(self.area);
        //
        //
        //
        f.render_text_raw(title, content_area[0]);
        f.render_rows_texts(rows, content_area[1]);
    }
}
