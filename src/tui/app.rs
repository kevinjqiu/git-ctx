use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    prelude::*,
    widgets::{
        block::{Position, Title},
        Block, Borders, List, ListDirection, ListState,
    },
};
use std::{
    cmp::{max, min},
    io::{self, stdout, BufRead, BufReader, Stdout},
    process::{Command, Stdio},
};

use ratatui::{backend::CrosstermBackend, Frame, Terminal};

use crate::Git;

#[derive(Debug, Default)]
pub struct App {
    branches: Vec<String>,
    current_branch: String,
    selected_index: i8,
    selected_branch: Option<String>,
    exit: bool,
}

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<Option<String>> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        terminal.clear()?;
        Ok(self.selected_branch.clone())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let mut state = ListState::default().with_selected(Some(self.selected_index as usize));
        let keybinding_help = Title::from(Line::from(vec![
            "<j>/<k> ".bold(),
            "or ".into(),
            "<up>/<down> ".bold(),
            "to navigate, ".into(),
            "<enter> ".bold(),
            "to select, ".into(),
            "<q>".bold(),
            " to quit".into(),
        ]));
        let list = List::new(self.branches.clone())
            .block(
                Block::default().borders(Borders::NONE).title(
                    keybinding_help
                        .alignment(Alignment::Left)
                        .position(Position::Bottom),
                ),
            )
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
            KeyCode::Enter => {
                self.selected_branch = Some(self.branches[self.selected_index as usize].clone());
                self.exit();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

fn init(limit: usize) -> io::Result<Tui> {
    enable_raw_mode()?;
    Terminal::with_options(
        CrosstermBackend::new(stdout()),
        TerminalOptions {
            viewport: Viewport::Inline(limit as u16),
        },
    )
}

fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    Ok(())
}

pub fn run_tui(limit: usize) -> io::Result<()> {
    let mut terminal = init(limit)?;
    let mut git = Git::default();
    let branches = git.get_recent_branches(limit).unwrap();
    let current_branch = git.get_current_branch().unwrap();

    let mut app = App {
        branches,
        current_branch,
        selected_index: 0,
        selected_branch: None,
        exit: false,
    };

    let app_result = app.run(&mut terminal)?;
    restore()?;

    match app_result {
        Some(branch) => {
            if branch != app.current_branch {
                println!("Switching to branch: {:?}", branch);

                let mut cmd = Command::new("git")
                    .args(["checkout", &branch])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();
                let stdout_reader = BufReader::new(cmd.stdout.as_mut().unwrap());
                let stdout_lines = stdout_reader.lines();

                for line in stdout_lines {
                    println!("{}", line.unwrap());
                }

                let stderr_reader = BufReader::new(cmd.stderr.as_mut().unwrap());
                let stderr_lines = stderr_reader.lines();

                for line in stderr_lines {
                    eprint!("{}", line.unwrap());
                }
            }
        }
        None => {}
    }
    Ok(())
}
