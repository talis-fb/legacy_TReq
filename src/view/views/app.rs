use crate::app::InputMode;

use crate::base::states::names::StatesNames;
use crate::base::web::response::ResponseStage;
use crate::view::components::counter_response_time::CounterResponseTime;
use crate::view::components::doc_reader::DocReader;
use crate::view::components::input_insert_mode::InputTextBlock;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::views::ViewStates;

use crate::view::style::Size;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

use super::environment::EnvironmentEditView;
use super::logs::LogView;

use super::request::RequestView;
use super::response::ResponseView;
use super::tabs_request::TabRequestView;

use crate::view::components::welcome_doc::WelcomeDoc;

pub struct AppView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl AppView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        TabRequestView::prepare_render(states, store);
        LogView::prepare_render(states, store);
        RequestView::prepare_render(states, store);
        ResponseView::prepare_render(states, store);
        EnvironmentEditView::prepare_render(states, store);
    }
}

impl Component for AppView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let screen_area = self.area;
        let store = self.store;

        let full_screen_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    // Request List Tab
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(2),
                ]
                .as_ref(),
            )
            .split(self.area);

        let sizes_layout = store.config.view.lock().unwrap();
        let (left, right) = sizes_layout.get_dimension_percentage();

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(left as u16),
                    Constraint::Percentage(right as u16),
                ]
                .as_ref(),
            )
            .split(full_screen_layout[1]);

        let tablist_requests_view = TabRequestView {
            area: full_screen_layout[0],
            store,
            states: self.states,
        };

        let request_view = RequestView {
            area: content_layout[0],
            store,
            states: self.states,
        };

        let response_view = ResponseView {
            area: content_layout[1],
            store,
            states: self.states,
        };

        // TODO: It should not use the same state of Response, only its area
        let welcome_doc_view = WelcomeDoc {
            area: content_layout[1],
            marked: store.current_state == StatesNames::ResponseBody
                || store.current_state == StatesNames::ResponseHeader,
        };

        let counter_response_time_view = CounterResponseTime {
            area: content_layout[1],
            marked: store.current_state == StatesNames::ResponseBody
                || store.current_state == StatesNames::ResponseHeader,
            time: 0.0,
        };

        let log_view = LogView {
            area: full_screen_layout[2],
            store,
            states: self.states,
        };

        let popup_component: Option<Box<dyn Component<Backend = BackendTuiRs>>> =
            match store.get_mode() {
                InputMode::Insert => Some(Box::new(InputTextBlock {
                    area: BackendTuiRs::create_absolute_centered_area(
                        Size::Percentage(60),
                        Size::Fixed(3),
                        screen_area,
                    ),
                    text: &store.input_buffer.value,
                    cursor: store.input_buffer.get_cursor(),
                })),
                InputMode::Help => Some(Box::new(DocReader {
                    area: BackendTuiRs::create_absolute_centered_area(
                        Size::Percentage(60),
                        Size::Percentage(75),
                        screen_area,
                    ),
                    doc_handler: store.doc_reader.as_ref().unwrap(),
                })),
                _ => None,
            };

        tablist_requests_view.render(f);
        request_view.render(f);
        log_view.render(f);

        let response_stage: Option<ResponseStage>;
        let response_time: f64;
        {
            let response_ref = store.get_response();
            let response = response_ref.lock().unwrap();
            response_stage = Some(response.stage);
            response_time = response.response_time;
        }
        match response_stage.unwrap() {
            ResponseStage::Empty => {
                let welcome_doc_view = WelcomeDoc {
                    area: content_layout[1],
                    marked: store.current_state == StatesNames::ResponseBody
                        || store.current_state == StatesNames::ResponseHeader,
                };
                welcome_doc_view.render(f)
            }
            ResponseStage::Waiting => {
                let counter_response_time_view = CounterResponseTime {
                    area: content_layout[1],
                    marked: store.current_state == StatesNames::ResponseBody
                        || store.current_state == StatesNames::ResponseHeader,
                    time: response_time,
                };
                counter_response_time_view.render(f)
            }
            _ => response_view.render(f),
        }

        match store.current_state {
            StatesNames::EditingGlobalEnv | StatesNames::EditingSessionEnv => {
                let environmet_view = EnvironmentEditView {
                    area: BackendTuiRs::create_absolute_centered_area(
                        Size::Percentage(80),
                        Size::Percentage(90),
                        screen_area,
                    ),
                    store,
                    states: self.states,
                };

                environmet_view.render(f);
            }
            _ => {}
        }

        if let Some(component) = popup_component {
            component.render(f);
        }
    }
}
