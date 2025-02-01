use std::borrow::Cow;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::keybinds::KeyBind;

const HL_STYLE: Style = Style::new().bg(Color::LightBlue).fg(Color::Yellow);
const HL_SYMBOL: &str = ">> ";

pub trait SearchableItem {
    fn to_list_item(&self, width: usize) -> ListItem<'_>;
    fn to_filter(&self) -> Cow<'_, str>;
}

pub struct SearchableList<T>
where
    T: SearchableItem,
{
    items: Vec<T>,
    curr_item_indices: Vec<usize>,
    selected: ListState,
    searching: bool,
    search_query: String,
}

impl<T: SearchableItem> SearchableList<T> {
    pub fn new(items: impl Into<Vec<T>>) -> Self {
        let items = items.into();
        let mut selected = ListState::default();
        if !items.is_empty() {
            selected.select(Some(0));
        }
        Self {
            items,
            curr_item_indices: vec![],
            selected,
            searching: false,
            search_query: String::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Percentage(100)])
            .split(area);

        let search_bar = {
            let search_text = if self.searching {
                let search_text = format!("{}█", &self.search_query);
                Span::raw(search_text)
            } else {
                let search_text = if self.search_query.is_empty() {
                    "Press / to search"
                } else {
                    self.search_query.as_str()
                };
                Span::styled(search_text, Style::default().fg(Color::Gray))
            };
            Paragraph::new(search_text).block(Block::default().borders(Borders::BOTTOM))
        };
        frame.render_widget(search_bar, chunks[0]);

        let list = {
            let item_width = chunks[1].width as usize - HL_SYMBOL.len();
            let mut items: Vec<ListItem> = if self.search_query.is_empty() {
                self.items
                    .iter()
                    .map(|i| i.to_list_item(item_width))
                    .collect()
            } else {
                self.curr_item_indices
                    .iter()
                    .map(|&i| self.items[i].to_list_item(item_width))
                    .collect()
            };
            if items.is_empty() {
                self.selected.select(None);
                items.push(ListItem::new(Span::raw("Nothing to see here")));
            }
            List::new(items)
                .highlight_style(HL_STYLE)
                .highlight_symbol(HL_SYMBOL)
                .highlight_spacing(HighlightSpacing::Always)
        };
        frame.render_stateful_widget(list, chunks[1], &mut self.selected);
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<&T> {
        if self.is_searching() {
            match key.code {
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.exit_search();
                }
                KeyCode::Esc => self.exit_search(),
                KeyCode::Enter => {
                    self.searching = false;
                    self.selected.select(Some(0));
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                    self.update_items();
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                    self.update_items();
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('/') => self.enter_search(),
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                    return if self.search_query.is_empty() {
                        self.selected.selected().map(|i| &self.items[i])
                    } else {
                        self.selected
                            .selected()
                            .map(|i| &self.items[self.curr_item_indices[i]])
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => self.select_next(),
                KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
                _ => {}
            }
        }
        None
    }

    pub fn keybinds(&self, has_prev: bool) -> Vec<KeyBind> {
        if self.is_searching() {
            vec![
                KeyBind::single_key("Ctrl+C", "Finish search"),
                KeyBind::new(vec!["Esc", "Enter"], "Exit search"),
                KeyBind::single_key("Backspace", "Remove last character"),
            ]
        } else {
            let mut binds = vec![
                KeyBind::single_key("/", "Enter Search"),
                KeyBind::new(vec!["l", "Right", "Enter"], "Select current item"),
                KeyBind::new(vec!["j", "Down"], "Select next item"),
                KeyBind::new(vec!["k", "Up"], "Select previous item"),
                // This keybind isn't part of the list, but all pages using the widget implement it
                KeyBind::new(vec!["q"], "Exit"),
            ];
            if has_prev {
                binds.push(KeyBind::new(vec!["h"], "Previous page"));
            }
            binds
        }
    }

    fn update_items(&mut self) {
        if self.search_query.is_empty() {
            self.curr_item_indices.clear();
        } else {
            let query = self.search_query.to_lowercase();
            self.curr_item_indices = self
                .items
                .iter()
                .enumerate()
                .filter_map(|(index, item)| {
                    (item.to_filter().to_lowercase().contains(&query)).then_some(index)
                })
                .collect();
        }
    }

    fn enter_search(&mut self) {
        self.searching = true;
        self.selected.select(None);
    }

    fn exit_search(&mut self) {
        self.searching = false;
        self.search_query.clear();
        self.curr_item_indices.clear();
        self.selected.select(Some(0));
    }

    pub fn is_searching(&self) -> bool {
        self.searching
    }

    fn select_next(&mut self) {
        if self
            .selected
            .selected()
            .is_some_and(|i| i < self.items.len() - 1)
        {
            self.selected.select_next();
        }
    }

    fn select_prev(&mut self) {
        if self.selected.selected().is_some_and(|i| i > 0) {
            self.selected.select_previous();
        }
    }
}
