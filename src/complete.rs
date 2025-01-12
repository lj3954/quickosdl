use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Flex, Layout, Rect},
    widgets::Paragraph,
    Frame,
};

use crate::{app::Action, keybinds::KeyBind};

pub struct CompletePage {}

impl CompletePage {
    pub fn new() -> Self {
        Self {}
    }
    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Null => None,
            _ => Some(Action::Exit),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let [centered] = Layout::vertical([Constraint::Length(1)])
            .flex(Flex::Center)
            .areas(area);
        let text = Paragraph::new("Complete! Press any key to exit.").centered();
        frame.render_widget(text, centered);
    }
    pub fn keybinds(&self) -> Vec<KeyBind> {
        vec![KeyBind::single_key("Any key", "Exit")]
    }
}
