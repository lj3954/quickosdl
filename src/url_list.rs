use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::{app::Action, keybinds::KeyBind, searchable_list::SearchableList};

pub struct UrlList {
    list: SearchableList<String>,
}

impl UrlList {
    pub fn new(urls: Vec<String>) -> Self {
        Self {
            list: SearchableList::new(urls),
        }
    }
    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') if !self.list.is_searching() => Some(Action::Exit),
            KeyCode::Char('h') if !self.list.is_searching() => Some(Action::PrevPage),
            _ => {
                self.list.handle_key(key);
                None
            }
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.list.draw(frame, area);
    }
    pub fn keybinds(&self) -> Vec<KeyBind> {
        self.list.keybinds(true)
    }
}
