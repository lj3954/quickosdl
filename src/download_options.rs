use std::borrow::Cow;

use quickget_core::data_structures::Config;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    text::Span,
    widgets::ListItem,
};

use crate::{
    app::{Action, Page},
    searchable_list::{SearchableItem, SearchableList},
};

const DOWNLOAD_OPTIONS: [DownloadOption; 2] = [DownloadOption::Download, DownloadOption::ListUrls];

impl SearchableItem for DownloadOption {
    fn to_list_item(&self, _: usize) -> ListItem<'_> {
        ListItem::from(Span::raw(self.as_ref()))
    }
    fn to_filter(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.as_ref())
    }
}

pub struct DownloadOptions {
    config: Config,
    list: SearchableList<DownloadOption>,
}

impl DownloadOptions {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            list: SearchableList::new(DOWNLOAD_OPTIONS),
        }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Char('q') if !self.list.is_searching() => Some(Action::Exit),
            KeyCode::Char('h') if !self.list.is_searching() => Some(Action::PrevPage),
            _ => self.list.handle_key(key).map(|option| match option {
                DownloadOption::Download => Action::NextPage(Page::Download),
                DownloadOption::ListUrls => Action::NextPage(Page::UrlList),
            }),
        }
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        self.list.draw(frame, area);
    }
}

enum DownloadOption {
    Download,
    ListUrls,
}

impl AsRef<str> for DownloadOption {
    fn as_ref(&self) -> &str {
        match self {
            DownloadOption::Download => "Download now",
            DownloadOption::ListUrls => "List URLs",
        }
    }
}
