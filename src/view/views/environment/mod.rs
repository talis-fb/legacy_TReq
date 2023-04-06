use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Style, Text, Texts};
use crate::view::views::environment::store::OpenedVars;
use crate::view::views::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod store;

static KEY_STATE: &str = "environment_vars__state";
// ------------------------

pub struct EnvironmentEditView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl EnvironmentEditView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        //
    }
}

impl Component for EnvironmentEditView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let state = &self.store.view.environment;

        f.render_clear_area(self.area);

        f.render_block_with_title_left(Texts::from_str(" [ESC] - QUIT "), self.area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Length(9), Constraint::Min(1)])
            .split(self.area);

        let layout_text_instruction = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Border top space
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1), // For line
            ])
            .split(layout[0]);

        f.render_text_raw_align_center(
            "All about how to use variables: https://github.com/talis-fb/TReq/wiki",
            layout_text_instruction[0],
        );
        f.render_text_raw_align_center(
            "[UP/DOWN] or [j/k] to navegate between sections of app",
            layout_text_instruction[1],
        );
        f.render_text_raw_align_center("[n] to add a new variable", layout_text_instruction[2]);
        f.render_text_raw_align_center("[d] to remove variable", layout_text_instruction[3]);
        f.render_text_raw_align_center("[e] to edit variable value", layout_text_instruction[4]);
        f.render_text_raw_align_center(
            "[TAB] to switch between global and session",
            layout_text_instruction[5],
        );

        match state.opened_section {
            OpenedVars::Session => {
                f.render_divider_with_text(
                    Texts::from_vec_text(vec![
                        Text::from_str_styled("Session", Style::from_color(Color::Yellow)),
                        Text::from_str(" / Global"),
                    ]),
                    layout_text_instruction[6],
                );
            }
            OpenedVars::Global => {
                f.render_divider_with_text(
                    Texts::from_vec_text(vec![
                        Text::from_str("Session / "),
                        Text::from_str_styled("Global", Style::from_color(Color::Yellow)),
                    ]),
                    layout_text_instruction[6],
                );
            }
        };

        let vars_keys = match state.opened_section {
            OpenedVars::Session => &state.vars_keys.session,
            OpenedVars::Global => &state.vars_keys.global,
        };

        let layout_content = Layout::default()
            .margin(1)
            .constraints(
                vars_keys
                    .iter()
                    .map(|f| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(layout[1]);

        vars_keys.iter().enumerate().for_each(|(i, var)| {
            let content = format!(r#"{} => "{}""#, var, {
                match state.opened_section {
                    OpenedVars::Session => self.store.environment.session.get(var).unwrap(),
                    OpenedVars::Global => self.store.environment.global.get(var).unwrap(),
                }
            });

            let current_selected = match state.opened_section {
                OpenedVars::Session => state.current_session_var,
                OpenedVars::Global => state.current_global_var,
            };

            if i == current_selected {
                f.render_text(
                    Texts {
                        body: vec![Text::from_str_styled(
                            &content,
                            Style::from_color(Color::Yellow),
                        )],
                    },
                    layout_content[i],
                );
            } else {
                f.render_text(Texts::from_str(&content), layout_content[i]);
            }
        });
    }
}
