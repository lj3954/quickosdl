use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct KeyBind {
    keys: Vec<&'static str>,
    action: &'static str,
}

impl KeyBind {
    pub fn new(keys: Vec<&'static str>, action: &'static str) -> Self {
        Self { keys, action }
    }
    pub fn single_key(key: &'static str, action: &'static str) -> Self {
        Self {
            keys: vec![key],
            action,
        }
    }
}

pub struct FinishedKeybinds {
    lines: Vec<Line<'static>>,
}

impl FinishedKeybinds {
    pub fn new(keybinds: impl IntoIterator<Item = KeyBind>, width: u16) -> Self {
        let mut line = Vec::new();
        let mut line_length = 0;

        let mut lines = vec![];
        let mut indices = vec![];

        for keybind in keybinds {
            let (bind, length) = render_keybind(keybind);
            if length > width {
                lines.push(Line::from(bind));
                continue;
            }
            let starting_index = if lines.is_empty() {
                (line_length + 1 + length < width).then(|| {
                    indices.push(line_length + 1);
                    line_length + 1
                })
            } else {
                indices
                    .iter()
                    .filter(|i| line_length + 1 > **i)
                    .nth(1)
                    .copied()
            };
            match starting_index {
                Some(starting_index) => {
                    lines.push(
                        std::iter::repeat(" ")
                            .take((starting_index - line_length) as usize)
                            .collect(),
                    );
                }
                None => {
                    lines.push(Line::from(line.clone()));
                    line.clear();
                    line_length = 0;
                }
            }
            line.extend(bind);
        }
        if !line.is_empty() {
            lines.push(Line::from(line));
        }
        Self { lines }
    }
    pub fn length(&self) -> u16 {
        self.lines.len() as _
    }
    pub fn draw(self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Paragraph::new(self.lines), area);
    }
}

fn render_keybind(keybind: KeyBind) -> (Vec<Span<'static>>, u16) {
    let sep = " / ";
    let mut length = 4;
    let mut spans = vec![];
    spans.push(Span::raw("["));
    let (last, rest) = keybind.keys.split_last().unwrap();
    for k in rest {
        length += k.len() + sep.len();
        spans.push(Span::styled(*k, Style::default().bold()));
        spans.push(Span::raw(sep));
    }
    length += last.len();
    spans.push(Span::styled(*last, Style::default().bold()));
    spans.push(Span::raw("]: "));

    spans.push(Span::styled(keybind.action, Style::default().bold()));
    length += keybind.action.len();
    (spans, length as _)
}
