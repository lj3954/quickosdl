use quickget_core::data_structures::Config;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    text::Span,
    widgets::ListItem,
};

use crate::{
    app::{Action, Page},
    download_options::DownloadOptions,
    searchable_list::{SearchableItem, SearchableList},
};

impl SearchableItem for Config {
    fn to_list_item(&self, _: usize) -> ListItem<'_> {
        ListItem::from(Span::raw(self.edition.as_deref().unwrap_or("None")))
    }
    fn to_filter(&self) -> std::borrow::Cow<'_, str> {
        self.edition.as_deref().unwrap_or("None").into()
    }
}

pub struct EditionSelection {
    list: SearchableList<Config>,
}

impl EditionSelection {
    pub fn new(list: Vec<Config>) -> Self {
        Self {
            list: SearchableList::new(list),
        }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') if !self.list.is_searching() => Some(Action::Exit),
            KeyCode::Char('h') if !self.list.is_searching() => Some(Action::PrevPage),
            _ => self.list.handle_key(key).map(|config| {
                Action::NextPage(Page::DownloadOptions(DownloadOptions::new(config.clone())))
            }),
        }
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        self.list.draw(frame, area);
    }
}
