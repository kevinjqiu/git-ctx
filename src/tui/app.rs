use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    prelude::*,
    widgets::{List, ListDirection, ListState},
};
use std::{
    cmp::{max, min},
    io::{self, stdout, Stdout},
};

use ratatui::{
    backend::CrosstermBackend, buffer::Buffer, layout::Rect, widgets::Widget, Frame, Terminal,
};

use crate::Git;

#[derive(Debug, Default)]
pub struct App {
    branches: Vec<String>,
    selected_index: i8,
    exit: bool,
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let mut state = ListState::default().with_selected(Some(self.selected_index as usize));
        let list = List::new(self.branches.clone())
            .highlight_style(Style::default())
            .highlight_symbol(">> ")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);
        StatefulWidget::render(list, frame.size(), frame.buffer_mut(), &mut state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected_index = max(0, self.selected_index - 1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected_index = min(self.selected_index + 1, (self.branches.len() - 1) as i8);
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(10),
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}

pub fn run_tui() -> io::Result<()> {
    let mut terminal = init()?;
    let mut app = App::default();
    let mut git = Git::default();
    app.branches = git.get_recent_branches(10).unwrap();

    let app_result = app.run(&mut terminal);
    restore()?;
    app_result
}
