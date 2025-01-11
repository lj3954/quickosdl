use std::borrow::Cow;

use crate::{
    app::{Action, Page},
    searchable_list::{SearchableItem, SearchableList},
};
use quickget_core::data_structures::Arch;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::ListItem,
    Frame,
};

const ARCHITECTURES: [Arch; 3] = [Arch::x86_64, Arch::aarch64, Arch::riscv64];

impl SearchableItem for Arch {
    fn to_list_item(&self) -> ListItem<'_> {
        ListItem::new(self.to_string())
    }
    fn to_filter(&self) -> Cow<'static, str> {
        Cow::Owned(self.to_string())
    }
}

pub struct ArchSelection {
    list: SearchableList<Arch>,
}

impl ArchSelection {
    pub fn new() -> Self {
        Self {
            list: SearchableList::new(ARCHITECTURES),
        }
    }
    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') if !self.list.is_searching() => Some(Action::Exit),
            _ => self
                .list
                .handle_key(key)
                .map(|_| Action::NextPage(Page::OSSelection)),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.list.draw(frame, area);
    }
}
