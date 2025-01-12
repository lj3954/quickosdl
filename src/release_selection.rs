use std::borrow::Cow;

use itertools::Itertools;
use quickget_core::data_structures::Config;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    text::Span,
    widgets::ListItem,
    Frame,
};

use crate::{
    app::{Action, Page},
    edition_selection::EditionSelection,
    searchable_list::{SearchableItem, SearchableList},
};

impl SearchableItem for String {
    fn to_list_item(&self, _: usize) -> ListItem<'_> {
        ListItem::from(Span::raw(self))
    }
    fn to_filter(&self) -> Cow<'_, str> {
        Cow::Borrowed(self)
    }
}

pub struct ReleaseSelection {
    configs: Vec<Config>,
    list: SearchableList<String>,
}

impl ReleaseSelection {
    pub fn new(configs: Vec<Config>) -> Self {
        let releases: Vec<String> = configs
            .iter()
            .map(|c| &c.release)
            .unique()
            .cloned()
            .collect();
        Self {
            configs,
            list: SearchableList::new(releases),
        }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') if !self.list.is_searching() => Some(Action::Exit),
            KeyCode::Char('h') if !self.list.is_searching() => Some(Action::PrevPage),
            _ => self.list.handle_key(key).map(|r| {
                let remaining_configs: Vec<Config> = self
                    .configs
                    .iter()
                    .filter(|c| &c.release == r)
                    .cloned()
                    .collect();
                if remaining_configs.len() == 1 {
                    Action::NextPage(Page::DownloadOptions)
                } else {
                    Action::NextPage(Page::EditionSelection(EditionSelection::new(
                        remaining_configs,
                    )))
                }
            }),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.list.draw(frame, area);
    }
}
