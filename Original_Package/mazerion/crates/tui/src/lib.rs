//! TUI using ratatui and crossterm.

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use mazerion_core::{list_calculators, Error, Result};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use std::io;

pub struct TuiApp {
    calculators: Vec<String>,
    selected: usize,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            calculators: list_calculators().iter().map(|s| s.to_string()).collect(),
            selected: 0,
        }
    }

    fn handle_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => return false,
            KeyCode::Down => {
                if self.selected < self.calculators.len().saturating_sub(1) {
                    self.selected = self.selected.saturating_add(1);
                }
            }
            KeyCode::Up => {
                self.selected = self.selected.saturating_sub(1);
            }
            _ => {}
        }
        true
    }

    fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(frame.area());

        let title = Paragraph::new("🍯 Mazerion Calculator (q to quit)")
            .block(Block::default().borders(Borders::ALL).title("Mazerion"));
        frame.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = self
            .calculators
            .iter()
            .enumerate()
            .map(|(i, calc)| {
                let style = if i == self.selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(calc.as_str()).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Calculators"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(list, chunks[1]);
    }
}

pub fn run() -> Result<()> {
    enable_raw_mode().map_err(|e| Error::Io(format!("Enable raw mode failed: {}", e)))?;
    io::stdout()
        .execute(EnterAlternateScreen)
        .map_err(|e| Error::Io(format!("Enter alternate screen failed: {}", e)))?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))
        .map_err(|e| Error::Io(format!("Terminal init failed: {}", e)))?;

    let mut app = TuiApp::new();
    let mut running = true;

    while running {
        terminal
            .draw(|f| app.render(f))
            .map_err(|e| Error::Io(format!("Draw failed: {}", e)))?;

        if event::poll(std::time::Duration::from_millis(100))
            .map_err(|e| Error::Io(format!("Event poll failed: {}", e)))?
        {
            if let Event::Key(key) = event::read().map_err(|e| Error::Io(e.to_string()))? {
                running = app.handle_key(key.code);
            }
        }
    }

    disable_raw_mode().map_err(|e| Error::Io(format!("Disable raw mode failed: {}", e)))?;
    io::stdout()
        .execute(LeaveAlternateScreen)
        .map_err(|e| Error::Io(format!("Leave alternate screen failed: {}", e)))?;

    Ok(())
}
