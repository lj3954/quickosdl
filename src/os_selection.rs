use std::{
    borrow::Cow,
    sync::{LazyLock, Mutex},
};

use quickget_core::{
    data_structures::{Arch, Config, Disk, Source, OS},
    ConfigSearch, ConfigSearchError,
};
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    text::{Line, Span},
    widgets::ListItem,
    Frame,
};
use tokio::runtime::Runtime;

use crate::{
    app::{Action, Page},
    error_display::ErrorDisplay,
    keybinds::KeyBind,
    release_selection::ReleaseSelection,
    searchable_list::{SearchableItem, SearchableList},
};

pub fn init_os_list() {
    _ = OS_LIST.as_ref();
}

static OS_LIST_POPULATED: Mutex<bool> = Mutex::new(false);
static OS_LIST: LazyLock<Result<Vec<OS>, ConfigSearchError>> = LazyLock::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let search = ConfigSearch::new_without_cache().await;
        *OS_LIST_POPULATED.lock().unwrap() = true;
        search.map(|s| s.into_os_list())
    })
});

impl SearchableItem for OS {
    fn to_list_item(&self, width: usize) -> ListItem {
        let mut lines = vec![Line::from(vec![Span::raw(&self.pretty_name)])];
        if let Some(description) = &self.description {
            let sep = "   ";
            let mut i = 0;
            while i < description.len() {
                let next_index = if i + width < description.len() {
                    description[i..(i + width - sep.len()).min(description.len() - 1)]
                        .rfind(' ')
                        .unwrap_or(description.len() - i - 1)
                } else {
                    description.len() - i - 1
                };
                lines.push(Line::from(vec![
                    Span::raw(sep),
                    Span::raw(&description[i..i + next_index]),
                ]));
                i += next_index + 1;
            }
        }

        ListItem::new(lines)
    }
    fn to_filter(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

pub struct OSSelection {
    arch: Arch,
    list: Option<SearchableList<OS>>,
}

impl OSSelection {
    pub fn new(arch: Arch) -> Self {
        let list = None;
        Self { arch, list }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        if let Some(list) = &mut self.list {
            match key.code {
                KeyCode::Char('q') if !list.is_searching() => Some(Action::Exit),
                KeyCode::Char('h') if !list.is_searching() => Some(Action::PrevPage),
                _ => list.handle_key(key).map(|os| {
                    Action::NextPage(Page::ReleaseSelection(ReleaseSelection::new(
                        os.releases.to_vec(),
                    )))
                }),
            }
        } else {
            if *OS_LIST_POPULATED.lock().unwrap() {
                match OS_LIST.as_ref() {
                    Ok(list) => {
                        let list: Vec<OS> = list
                            .iter()
                            .cloned()
                            .map(|mut os| {
                                os.releases.retain(has_only_wanted_sources);
                                os.releases.retain(|c| correct_arch(c, &self.arch));
                                os
                            })
                            .filter(|os| !os.releases.is_empty())
                            .collect();
                        self.list = Some(SearchableList::new(list));
                    }
                    Err(e) => {
                        return Some(Action::NextPage(Page::Error(ErrorDisplay::new(vec![
                            e.to_string()
                        ]))))
                    }
                }
            }
            match key.code {
                KeyCode::Char('h') => Some(Action::PrevPage),
                KeyCode::Char('q') => Some(Action::Exit),
                _ => None,
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        if let Some(list) = &mut self.list {
            list.draw(frame, area);
        } else {
            let text = vec![Span::raw("Loading...")];
            frame.render_widget(Line::from(text), area);
        }
    }
    pub fn keybinds(&self) -> Vec<KeyBind> {
        match &self.list {
            Some(list) => list.keybinds(true),
            None => vec![
                KeyBind::single_key("q", "Exit"),
                KeyBind::single_key("h", "Previous page"),
            ],
        }
    }
}

fn correct_arch(config: &Config, arch: &Arch) -> bool {
    &config.arch == arch
}

fn has_only_wanted_sources(config: &Config) -> bool {
    (config.disk_images.is_none() || config.disk_images == Some(vec![Disk::default()]))
        && config
            .iso
            .iter()
            .chain(config.img.iter())
            .all(|s| matches!(s, Source::Web(_)))
}
