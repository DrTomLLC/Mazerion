use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use ratatui::backend::Backend;
// Force calculators to register
use mazerion_core as _;

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> where std::io::Error: From<<B as Backend>::Error> {
    let calculators = mazerion_core::get_all_calculators();
    let mut selected: usize = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            let title = Paragraph::new("🍯 Mazerion TUI - Press 'q' to quit")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = calculators
                .iter()
                .enumerate()
                .map(|(i, calc)| {
                    let style = if i == selected {
                        Style::default().bg(Color::Blue).fg(Color::White)
                    } else {
                        Style::default()
                    };
                    ListItem::new(vec![Line::from(vec![
                        Span::raw(calc.name()),
                        Span::raw(" - "),
                        Span::styled(calc.description(), Style::default().fg(Color::Gray)),
                    ])])
                        .style(style)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Calculators").borders(Borders::ALL));
            f.render_widget(list, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {
                    if selected < calculators.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}