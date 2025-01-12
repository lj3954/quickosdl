use std::{
    io::{self, Stdout},
    time::Duration,
};

use app::App;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    prelude::CrosstermBackend,
    Terminal,
};

mod app;
mod arch_selection;
mod complete;
mod download;
mod download_options;
mod edition_selection;
mod error_display;
mod keybinds;
mod os_selection;
mod release_selection;
mod searchable_list;
mod url_list;

fn main() -> io::Result<()> {
    let mut app = App::new();

    let mut terminal = ratatui::try_init()?;
    app.run(&mut terminal)?;
    ratatui::try_restore()
}

impl App {
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        std::thread::spawn(move || {
            os_selection::init_os_list();
        });
        loop {
            terminal.draw(|f| self.draw(f))?;

            if !event::poll(Duration::from_millis(10))? {
                // Send a null key to ensure consistent refreshes
                // Cost appears to be negligible
                if self.handle_key(&KeyEvent::new(KeyCode::Null, KeyModifiers::NONE)) {
                    break;
                }
                continue;
            }

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press && key.kind != KeyEventKind::Release {
                    continue;
                }
                if self.handle_key(&key) {
                    break;
                }
            }
        }
        Ok(())
    }
}
