use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    arch_selection::ArchSelection,
    complete::CompletePage,
    download::DownloadPage,
    download_options::DownloadOptions,
    edition_selection::EditionSelection,
    error_display::ErrorDisplay,
    keybinds::{FinishedKeybinds, KeyBind},
    os_selection::OSSelection,
    release_selection::ReleaseSelection,
    url_list::UrlList,
};

pub struct App {
    page_stack: Vec<Page>,
}

impl App {
    pub fn new() -> Self {
        Self {
            page_stack: vec![Page::ArchSelection(ArchSelection::new())],
        }
    }

    pub fn current_page(&mut self) -> &mut Page {
        self.page_stack.last_mut().unwrap()
    }

    pub fn pop_page(&mut self) {
        self.page_stack.pop();
    }

    pub fn push_page(&mut self, page: Page) {
        self.page_stack.push(page);
    }

    pub fn title(&self) -> Line<'static> {
        let page_names: Vec<&str> = self.page_stack.iter().map(|p| p.page_name()).collect();
        let (last, rest) = page_names.split_last().unwrap();

        let title = Span::styled("QuickOSDL", Style::default().bold());
        let prev_navigation = Span::raw(format!(
            ": Start -> {}",
            rest.iter().flat_map(|n| [n, " -> "]).collect::<String>()
        ));
        let curr_page = Span::styled(*last, Style::default().bold());

        Line::from(vec![
            Span::raw(" "),
            title,
            prev_navigation,
            curr_page,
            Span::raw(" "),
        ])
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let block = Block::bordered().title(self.title());
        let inner_area = block.inner(frame.area());

        let keybinds = self.current_page().keybinds();
        let keybinds = FinishedKeybinds::new(keybinds, inner_area.width);
        let length = keybinds.length();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(length)])
            .split(inner_area);

        let kb_block = Block::default().borders(Borders::TOP).title(" Keybinds ");
        frame.render_widget(kb_block, chunks[1]);

        keybinds.draw(frame, chunks[1]);

        self.current_page().draw(frame, chunks[0]);
        frame.render_widget(block, frame.area());
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> bool {
        if let Some(action) = self.current_page().handle_key(key) {
            match action {
                Action::Exit => return true,
                Action::PrevPage => self.pop_page(),
                Action::NextPage(page) => self.push_page(page),
            }
        }
        false
    }
}

pub enum Action {
    Exit,
    PrevPage,
    NextPage(Page),
}

pub enum Page {
    ArchSelection(ArchSelection),
    OSSelection(OSSelection),
    ReleaseSelection(ReleaseSelection),
    EditionSelection(EditionSelection),
    DownloadOptions(DownloadOptions),
    Download(DownloadPage),
    UrlList(UrlList),
    Complete(CompletePage),
    Error(ErrorDisplay),
}

impl Page {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            Page::ArchSelection(arch_selection) => arch_selection.draw(frame, area),
            Page::OSSelection(os_selection) => os_selection.draw(frame, area),
            Page::ReleaseSelection(release_selection) => release_selection.draw(frame, area),
            Page::EditionSelection(edition_selection) => edition_selection.draw(frame, area),
            Page::DownloadOptions(download_options) => download_options.draw(frame, area),
            Page::Download(download_page) => download_page.draw(frame, area),
            Page::UrlList(url_list) => url_list.draw(frame, area),
            Page::Complete(complete_page) => complete_page.draw(frame, area),
            Page::Error(error_display) => error_display.draw(frame, area),
        }
    }
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match self {
            Page::ArchSelection(arch_selection) => arch_selection.handle_key(key),
            Page::OSSelection(os_selection) => os_selection.handle_key(key),
            Page::ReleaseSelection(release_selection) => release_selection.handle_key(key),
            Page::EditionSelection(edition_selection) => edition_selection.handle_key(key),
            Page::DownloadOptions(download_options) => download_options.handle_key(key),
            Page::Download(download_page) => download_page.handle_key(key),
            Page::UrlList(url_list) => url_list.handle_key(key),
            Page::Complete(complete_page) => complete_page.handle_key(key),
            Page::Error(error_display) => error_display.handle_key(key),
        }
    }

    fn page_name(&self) -> &'static str {
        match self {
            Page::ArchSelection(_) => "Arch",
            Page::OSSelection(_) => "OS",
            Page::ReleaseSelection(_) => "Release",
            Page::EditionSelection(_) => "Edition",
            Page::DownloadOptions(_) => "Download Options",
            Page::Download(_) => "Download",
            Page::UrlList(_) => "URLs",
            Page::Complete(_) => "Complete",
            Page::Error(_) => "Error",
        }
    }

    fn keybinds(&self) -> Vec<KeyBind> {
        match self {
            Page::ArchSelection(arch_selection) => arch_selection.keybinds(),
            Page::OSSelection(os_selection) => os_selection.keybinds(),
            Page::ReleaseSelection(release_selection) => release_selection.keybinds(),
            Page::EditionSelection(edition_selection) => edition_selection.keybinds(),
            Page::DownloadOptions(download_options) => download_options.keybinds(),
            Page::Download(download_page) => download_page.keybinds(),
            Page::UrlList(url_list) => url_list.keybinds(),
            Page::Complete(complete_page) => complete_page.keybinds(),
            Page::Error(error_display) => error_display.keybinds(),
        }
    }
}
