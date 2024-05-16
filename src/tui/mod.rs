mod app;

use crate::Result;
use crossterm::event;
use crossterm::event::{KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;
use std::io::stdout;

pub fn run_tui() -> Result<()> {
    // stdout().execute(EnterAlternateScreen)?;
    // enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    // terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();

            frame.render_widget(Paragraph::new("Hello Ratatui! (press 'q' to exit)"), area)
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // stdout().execute(LeaveAlternateScreen)?;
    // disable_raw_mode()?;
    Ok(())
}
