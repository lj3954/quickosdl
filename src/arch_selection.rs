use std::borrow::Cow;

use quickget_core::data_structures::Arch;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    widgets::ListItem,
    Frame,
};

use crate::{
    app::{Action, Page},
    keybinds::KeyBind,
    os_selection::OSSelection,
    searchable_list::{SearchableItem, SearchableList},
};

const ARCHITECTURES: [Arch; 3] = [Arch::x86_64, Arch::aarch64, Arch::riscv64];

impl SearchableItem for Arch {
    fn to_list_item(&self, _: usize) -> ListItem<'_> {
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
                .map(|arch| Action::NextPage(Page::OSSelection(OSSelection::new(arch.clone())))),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.list.draw(frame, area);
    }

    pub fn keybinds(&self) -> Vec<KeyBind> {
        self.list.keybinds(false)
    }
}
