use std::{
    borrow::Cow,
    fmt::Display,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use quickget_core::data_structures::WebSource;
use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::Gauge,
    Frame,
};
use size::Size;
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::{
    app::{Action, Page},
    complete::CompletePage,
    error_display::ErrorDisplay,
    keybinds::KeyBind,
};

pub struct DownloadPage {
    rt: Runtime,
    has_failed_download: bool,
    downloads: Vec<Download>,
}

impl DownloadPage {
    pub fn new(sources: impl Iterator<Item = WebSource>) -> Self {
        let rt = Runtime::new().unwrap();
        let downloads = sources.into_iter().map(|s| Download::new(&rt, s)).collect();
        Self {
            rt,
            has_failed_download: false,
            downloads,
        }
    }

    pub fn handle_key(&mut self, key: &KeyEvent) -> Option<Action> {
        if let KeyCode::Null = key.code {
            return None;
        }
        let mut all_complete = true;
        let mut errors = vec![];
        for download in self.downloads.iter() {
            match &download.status {
                DownloadStatus::Failed(e) => match e {
                    DownloadError::Reqwest(e) => errors.push(e.to_string()),
                    DownloadError::Io(e) => errors.push(e.to_string()),
                },
                DownloadStatus::InProgress => all_complete = false,
                DownloadStatus::Complete => {}
            }
        }
        if !errors.is_empty() {
            self.downloads.iter().for_each(Download::cancel);
            Some(Action::NextPage(Page::Error(ErrorDisplay::new(errors))))
        } else if all_complete {
            Some(Action::NextPage(Page::Complete(CompletePage::new())))
        } else {
            None
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(self.downloads.iter().map(|_| Constraint::Max(5)))
            .split(area);
        self.downloads.iter_mut().enumerate().for_each(|(i, d)| {
            if let DownloadStatus::InProgress = d.status {
                if d.handle.as_ref().unwrap().is_finished() {
                    let handle = d.handle.take().unwrap();
                    self.rt.block_on(async {
                        let result = handle.await;
                        d.status = match result {
                            Ok(Err(e)) => {
                                self.has_failed_download = true;
                                DownloadStatus::Failed(e)
                            }
                            Ok(_) => DownloadStatus::Complete,
                            Err(e) => panic!("Error spawning thread: {:?}", e),
                        };
                    })
                }
            }
            let (total, current) = (
                d.total_size.load(Ordering::Relaxed),
                d.current_size.load(Ordering::Relaxed),
            );
            let ratio = if total == 0 || !matches!(d.status, DownloadStatus::InProgress) {
                1.0
            } else {
                current as f64 / total as f64
            };

            let (color, text, text_color) = match &d.status {
                DownloadStatus::Failed(e) => (
                    Color::Red,
                    Cow::Owned(format!("Download Failed: {e}")),
                    Color::Black,
                ),
                DownloadStatus::InProgress => (
                    Color::Blue,
                    Cow::Owned(format!(
                        "{:.2}% ({}/{})",
                        ratio * 100.0,
                        Size::from_bytes(current),
                        Size::from_bytes(total)
                    )),
                    Color::White,
                ),
                DownloadStatus::Complete => (
                    Color::Green,
                    Cow::Owned(format!("Download complete ({})", Size::from_bytes(current))),
                    Color::Black,
                ),
            };
            let text = Span::styled(text, Style::default().fg(text_color));

            let gauge = Gauge::default().ratio(ratio).gauge_style(color).label(text);
            frame.render_widget(gauge, chunks[i]);
        });
    }

    pub fn keybinds(&self) -> Vec<KeyBind> {
        if self.has_failed_download {
            vec![KeyBind::single_key("Any key", "Exit downloads")]
        } else {
            vec![]
        }
    }
}

#[allow(dead_code)]
enum DownloadError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DownloadError::Reqwest(e) => write!(f, "{}", e),
            DownloadError::Io(e) => write!(f, "{}", e),
        }
    }
}

enum DownloadStatus {
    Failed(DownloadError),
    Complete,
    InProgress,
}

struct Download {
    status: DownloadStatus,
    handle: Option<JoinHandle<Result<(), DownloadError>>>,
    current_size: Arc<AtomicU64>,
    total_size: Arc<AtomicU64>,
}

impl Download {
    fn new(rt: &Runtime, source: WebSource) -> Self {
        let total_size = Arc::new(AtomicU64::new(0));
        let current_size = Arc::new(AtomicU64::new(0));

        let as_total_size = total_size.clone();
        let as_current_size = current_size.clone();
        let handle = rt.spawn(async move {
            let mut response = reqwest::get(&source.url)
                .await
                .map_err(DownloadError::Reqwest)?;
            let size = response.content_length().unwrap_or(0);
            as_total_size.store(size, Ordering::Relaxed);

            let filename = source
                .file_name
                .as_deref()
                .unwrap_or_else(|| response.url().path_segments().unwrap().last().unwrap());
            let mut file = File::create_new(filename).map_err(DownloadError::Io)?;

            while let Some(chunk) = response.chunk().await.map_err(DownloadError::Reqwest)? {
                file.write_all(&chunk).map_err(DownloadError::Io)?;
                as_current_size.fetch_add(chunk.len() as u64, Ordering::Relaxed);
            }
            Ok(())
        });

        Self {
            status: DownloadStatus::InProgress,
            handle: Some(handle),
            current_size,
            total_size,
        }
    }
    fn cancel(&self) {
        if let Some(handle) = self.handle.as_ref() {
            handle.abort();
        }
    }
}
