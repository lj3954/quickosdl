use std::{
    borrow::Cow,
    fs::File,
    io::Write,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use quickget_core::data_structures::WebSource;
use ratatui::{
    crossterm::event::KeyEvent,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::Gauge,
    Frame,
};
use size::Size;
use tokio::{runtime::Runtime, task::JoinHandle};

use crate::app::{Action, Page};

pub struct DownloadPage {
    rt: Runtime,
    downloads: Vec<Download>,
}

impl DownloadPage {
    pub fn new(sources: impl Iterator<Item = WebSource>) -> Self {
        let rt = Runtime::new().unwrap();
        let downloads = sources.into_iter().map(|s| Download::new(&rt, s)).collect();
        Self { rt, downloads }
    }

    pub fn handle_key(&mut self, _: &KeyEvent) -> Option<Action> {
        let mut all_complete = true;
        let mut errors = vec![];
        for download in self.downloads.iter() {
            match &download.status {
                DownloadStatus::Failed(e) => errors.push(e),
                DownloadStatus::InProgress => all_complete = false,
                DownloadStatus::Complete => {}
            }
        }
        if !errors.is_empty() {
            Some(Action::NextPage(Page::Error))
        } else if all_complete {
            Some(Action::NextPage(Page::Complete))
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
                            Ok(Err(e)) => DownloadStatus::Failed(e),
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
                0.0
            } else {
                current as f64 / total as f64
            };

            let (color, text, text_color) = match d.status {
                DownloadStatus::Failed(_) => {
                    (Color::Red, Cow::Borrowed("Download Failed"), Color::Black)
                }
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
}

enum DownloadError {
    Reqwest(reqwest::Error),
    Io(std::io::Error),
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
}
