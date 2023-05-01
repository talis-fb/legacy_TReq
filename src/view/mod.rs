use crate::base::stores::MainStore;

pub mod components;
pub mod help;
pub mod renderer;
pub mod style;
pub mod ui;
pub mod views;

pub trait UiTrait {
    fn restart(&mut self);
    fn close(&mut self);

    fn render(&mut self, data_store: &MainStore);
}
