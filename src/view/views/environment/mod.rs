use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Style, Text, Texts};
use crate::view::views::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};
use crate::view::views::environment::store::{State, Var, EnvironmentVars, OpenedVars};

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
        // let state_json = states.entry(KEY_STATE.to_string()).or_default();
        // let last_state: Result<State, serde_json::Error> = serde_json::from_str(state_json);
        //
        // let global_vars: Vec<Var> = store
        //     .environment
        //     .global
        //     .iter()
        //     .map(|(key, _)| Var { key: key.clone() })
        //     .collect();
        //
        // let session_vars: Vec<Var> = store
        //     .environment
        //     .session
        //     .iter()
        //     .map(|(key, _)| Var { key: key.clone() })
        //     .collect();
        //
        // let state = State {
        //     opened_section: OpenedVars::Session,
        //     active_var: 0,
        //     vars: EnvironmentVars {
        //         global: global_vars,
        //         session: session_vars,
        //     },
        // };
        //
        // states.insert(
        //     KEY_STATE.to_string(),
        //     serde_json::to_string(&state).unwrap(),
        // );
    }
}

impl Component for EnvironmentEditView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        // let state: State = serde_json::from_str(self.states.get(KEY_STATE).unwrap()).unwrap();
        let state = &self.store.view.environment;

        f.render_clear_area(self.area);

        f.render_block_with_title_left(Texts::from_str(" [ESC] - QUIT "), self.area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(self.area);

        let layout_text_instruction = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1), // For line
            ])
            .split(layout[0]);

        f.render_text_raw_align_center("Opa", layout_text_instruction[0]);

        match state.opened_section {
            OpenedVars::Session => {
                f.render_divider_with_text(
                    Texts::from_vec_text(vec![
                        Text::from_str_styled("Session", Style::from_color(Color::Yellow)),
                        Text::from_str(" / Global"),
                    ]),
                    layout_text_instruction[3],
                );
            }
            OpenedVars::Global => {
                f.render_divider_with_text(
                    Texts::from_vec_text(vec![
                        Text::from_str("Session / "),
                        Text::from_str_styled("Global", Style::from_color(Color::Yellow)),
                    ]),
                    layout_text_instruction[3],
                );
            }
        };

        let vars_keys = match state.opened_section {
            OpenedVars::Session => &state.vars.session,
            OpenedVars::Global => &state.vars.global,
        };

        let vars_keys = vars_keys.split_at(0).1;

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
            let ff = format!(r#"{} => "{}""#, var.key, self.store.environment.session.get(&var.key).unwrap());

            if i == state.active_var {
                f.render_text(
                    Texts {
                        body: vec![Text::from_str_styled(&ff, Style::from_color(Color::Yellow))],
                    },
                    layout_content[i],
                );
            } else {
                f.render_text(Texts::from_str(&ff), layout_content[i]);
            }
        });

        // let mut texts_showed = vec![];
        //
        // map.into_iter().for_each(|(k, v)| {
        //     texts_showed.push(Texts::from_str(" "));
        //     texts_showed.push(Texts::from_str(&(" => ".to_owned() + &v.to_owned())));
        // });
    }
}
