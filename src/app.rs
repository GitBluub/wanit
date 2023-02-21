use std::io::{self, Stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    Frame, Terminal,
};
use wanikani_rs::wanikani_client::WanikaniClient;

use crate::error::WanitError;

pub enum Mode {
    Dashboard,
    Lesson,
    Review,
}

pub struct Wanit {
    pub mode: Mode,
    pub client: WanikaniClient,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Wanit {
    pub fn new(client: WanikaniClient) -> Self {
        let mut terminal = Wanit::start_tui().unwrap();
        Self {
            mode: Mode::Dashboard,
            client,
            terminal,
        }
    }

    pub fn start_tui() -> Result<Terminal<CrosstermBackend<Stdout>>, WanitError> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(terminal)
    }

    pub fn stop_tui() -> Result<(), WanitError> {
        let backend = CrosstermBackend::new(io::stdout());
        let mut terminal = Terminal::new(backend)?;
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        f: &mut Frame<impl Backend>,
    ) {
        terminal.draw(|f| self.draw_dashboard(f).unwrap());
    }

    pub fn draw_dashboard(&mut self, f: &mut Frame<impl Backend>) -> Result<(), WanitError> {
        Ok(())
    }
}
