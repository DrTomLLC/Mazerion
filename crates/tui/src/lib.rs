use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mazerion_core::{get_all_calculators, Calculator};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

enum AppState {
    List,
    CalculatorInput(String), // calculator id
}

struct App {
    calculators: Vec<Box<dyn Calculator>>,
    selected: usize,
    state: AppState,
}

impl App {
    fn new() -> Self {
        Self {
            calculators: get_all_calculators(),
            selected: 0,
            state: AppState::List,
        }
    }
}

pub fn run() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    io::Error: From<<B as Backend>::Error>,
{
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.state {
                AppState::List => {
                    if handle_list_input(key, app) {
                        return Ok(());
                    }
                }
                AppState::CalculatorInput(_) => {
                    if handle_input_mode(key, app) {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn handle_list_input(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Down | KeyCode::Char('j') => {
            app.selected = app.selected.saturating_add(1).min(app.calculators.len().saturating_sub(1));
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.selected = app.selected.saturating_sub(1);
        }
        KeyCode::Home => {
            app.selected = 0;
        }
        KeyCode::End => {
            app.selected = app.calculators.len().saturating_sub(1);
        }
        KeyCode::Enter => {
            if let Some(calc) = app.calculators.get(app.selected) {
                app.state = AppState::CalculatorInput(calc.id().to_string());
            }
        }
        _ => {}
    }
    false
}

fn handle_input_mode(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Esc => {
            app.state = AppState::List;
        }
        _ => {}
    }
    false
}

fn ui(f: &mut Frame, app: &App) {
    match &app.state {
        AppState::List => render_list(f, app),
        AppState::CalculatorInput(calc_id) => render_calculator_input(f, app, calc_id),
    }
}

fn render_list(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Title
    let title = Paragraph::new(vec![Line::from(vec![
        Span::styled("🍯 ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "Mazerion TUI",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(" - {} Calculators", app.calculators.len()),
            Style::default().fg(Color::Gray),
        ),
    ])])
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Calculator list
    let items: Vec<ListItem> = app
        .calculators
        .iter()
        .enumerate()
        .map(|(i, calc)| {
            let style = if i == app.selected {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(format!("{:2}. ", i + 1), Style::default().fg(Color::Yellow)),
                Span::styled(calc.name(), style.clone()),
                Span::raw(" - "),
                Span::styled(calc.description(), Style::default().fg(Color::Gray)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Select Calculator (↑/↓ or j/k, Enter to open, q to quit)"),
    );
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("↑/↓ j/k", Style::default().fg(Color::Cyan)),
        Span::raw(": Navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(": Open  "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(": Quit"),
    ]))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_calculator_input(f: &mut Frame, app: &App, calc_id: &str) {
    let calc = app
        .calculators
        .iter()
        .find(|c| c.id() == calc_id)
        .expect("Calculator not found");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(0)])
        .split(f.area());

    // Header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("📋 ", Style::default().fg(Color::Yellow)),
            Span::styled(
                calc.name(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(calc.description(), Style::default().fg(Color::Gray))),
    ])
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Input form (placeholder)
    let content = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Calculator input form coming soon!",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Press ESC to go back", Style::default().fg(Color::Gray))),
    ])
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(content, chunks[1]);
}