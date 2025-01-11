use std::io::{self, Stdout};

use app::App;
use ratatui::{
    crossterm::event::{self, Event, KeyEventKind},
    prelude::CrosstermBackend,
    Terminal,
};

mod app;
mod arch_selection;
mod searchable_list;

fn main() -> io::Result<()> {
    let mut app = App::new();

    let mut terminal = ratatui::try_init()?;
    app.run(&mut terminal)?;
    ratatui::try_restore()
}

impl App {
    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.draw(f))?;
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
