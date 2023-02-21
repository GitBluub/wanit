mod app;
pub mod error;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use error::WanitError;
use std::{io, thread, time::Duration};
use tui::{
    backend::Backend,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use wanikani_rs::wanikani_client::WanikaniClient;

use crate::app::Wanit;

fn draw_dashboard<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.size());
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let review = Block::default().title("Review").borders(Borders::ALL);
    let lesson = Block::default().title("Lesson").borders(Borders::ALL);
    let inner_lesson = lesson.inner(left_chunks[0]);
    let lesson_p = Paragraph::new("45");
    f.render_widget(lesson_p, inner_lesson);
    f.render_widget(lesson, left_chunks[0]);
    f.render_widget(review, left_chunks[1]);
}

#[tokio::main]
async fn main() -> Result<(), WanitError> {
    let client = WanikaniClient::default();
    let wanit = Wanit::new(client);
    let summary = wanit.client.get_summary().await?;

    dbg!(summary);
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| draw_dashboard(f))?;

    thread::sleep(Duration::from_millis(5000));

    Wanit::stop_tui()?;
    // restore terminal
    Ok(())
}
