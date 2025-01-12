use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Block,
    Frame,
};

use crate::{
    arch_selection::ArchSelection, edition_selection::EditionSelection, os_selection::OSSelection,
    release_selection::ReleaseSelection,
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
        self.current_page().draw(frame, inner_area);
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
    DownloadOptions,
    Download,
    UrlList,
    Complete,
    Error,
}

impl Page {
    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            Page::ArchSelection(arch_selection) => arch_selection.draw(frame, area),
            Page::OSSelection(os_selection) => os_selection.draw(frame, area),
            Page::ReleaseSelection(release_selection) => release_selection.draw(frame, area),
            Page::EditionSelection(edition_selection) => edition_selection.draw(frame, area),
            _ => unimplemented!(),
        }
    }
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match self {
            Page::ArchSelection(arch_selection) => arch_selection.handle_key(key),
            Page::OSSelection(os_selection) => os_selection.handle_key(key),
            Page::ReleaseSelection(release_selection) => release_selection.handle_key(key),
            Page::EditionSelection(edition_selection) => edition_selection.handle_key(key),
            _ => unimplemented!(),
        }
    }

    fn page_name(&self) -> &'static str {
        match self {
            Page::ArchSelection(_) => "Arch",
            Page::OSSelection(_) => "OS",
            Page::ReleaseSelection(_) => "Release",
            Page::EditionSelection(_) => "Edition",
            Page::DownloadOptions => "Download Options",
            Page::Download => "Download",
            Page::UrlList => "URLs",
            Page::Complete => "Complete",
            Page::Error => "Error",
        }
    }
}
