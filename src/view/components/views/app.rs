use crate::app::InputMode;
use crate::base::states::names::StatesNames;
use crate::base::web::request::METHODS;
use crate::view::components::block_text::BlockText;
use crate::view::components::doc_reader::DocReader;
use crate::view::components::input_block::InputTextBlock;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Size, Texts};
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

use super::logs::LogView;
use super::request::request_edition_view::{RequestEditionView, StatesReqEditionView};
use super::request::RequestView;
use super::response::ResponseView;
use super::tabs_request::TabRequestView;

pub struct AppView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
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
        };

        let request_view = RequestView {
            area: content_layout[0],
            store,
        };

        let response_view = ResponseView {
            area: content_layout[1],
            store,
        };

        let log_view = LogView {
            area: full_screen_layout[2],
            store,
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
        response_view.render(f);
        log_view.render(f);

        if let Some(component) = popup_component {
            component.render(f);
        }
    }
}
