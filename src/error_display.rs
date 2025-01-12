use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::Action;

pub struct ErrorDisplay {
    errors: Vec<String>,
}

impl ErrorDisplay {
    pub fn new(errors: Vec<String>) -> Self {
        Self { errors }
    }
    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        match key.code {
            KeyCode::Null => None,
            _ => Some(Action::Exit),
        }
    }
    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let [centered] = Layout::vertical([Constraint::Length(self.errors.len() as u16 + 3)])
            .flex(Flex::Center)
            .areas(area);
        let mut error_lines = Vec::with_capacity(self.errors.len() + 3);
        error_lines.push(Line::from(vec![Span::styled(
            "Error:",
            Style::default().bold().fg(Color::Red),
        )]));

        error_lines.extend(
            self.errors
                .iter()
                .map(|e| Line::from(vec![Span::raw(e.to_string())])),
        );
        error_lines.push(Line::from(vec![]));
        error_lines.push(Line::from(vec![Span::styled(
            "Press any key to exit.",
            Style::default().bold(),
        )]));
        let text = Paragraph::new(error_lines).centered();
        frame.render_widget(text, centered);
    }
}
