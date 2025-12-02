//! Production TUI with MCL organization

use crossterm::{event::{self, Event, KeyCode}, execute,
                terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout},
              style::{Color, Modifier, Style}, text::{Line, Span},
              widgets::{Block, Borders, List, ListItem, Paragraph}, Terminal};
use std::io;
use mazerion_core::{get_calculators_by_category, VALID_CATEGORIES};

pub fn run() -> io::Result<()> {
    if let Err(e) = mazerion_calculators::init() {
        return Err(io::Error::new(io::ErrorKind::Other, e));
    }
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

struct AppState {
    current_category_index: usize,
    selected_calc_index: usize,
    categories: Vec<String>,
}

impl AppState {
    fn new() -> Self {
        let by_category = get_calculators_by_category();
        let categories: Vec<String> = VALID_CATEGORIES.iter()
            .filter(|cat| by_category.contains_key(*cat as &str))
            .map(|s| s.to_string()).collect();
        Self { current_category_index: 0, selected_calc_index: 0, categories }
    }
    fn current_category(&self) -> &str { &self.categories[self.current_category_index] }
    fn next_category(&mut self) {
        if self.current_category_index < self.categories.len() - 1 {
            self.current_category_index += 1; self.selected_calc_index = 0;
        }
    }
    fn prev_category(&mut self) {
        if self.current_category_index > 0 {
            self.current_category_index -= 1; self.selected_calc_index = 0;
        }
    }
    fn next_calculator(&mut self, max: usize) {
        if self.selected_calc_index < max - 1 { self.selected_calc_index += 1; }
    }
    fn prev_calculator(&mut self) {
        if self.selected_calc_index > 0 { self.selected_calc_index -= 1; }
    }
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()>
where io::Error: From<<B as ratatui::backend::Backend>::Error> {
    let mut state = AppState::new();
    let by_category = get_calculators_by_category();
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default().direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3),
                    Constraint::Min(0), Constraint::Length(4)])
                .split(f.area());
            let title = Paragraph::new("🍯 Mazerion MCL")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(title, chunks[0]);
            let category_tabs: Vec<Span> = state.categories.iter().enumerate()
                .map(|(i, cat)| {
                    let count = by_category.get(cat.as_str()).map(|v| v.len()).unwrap_or(0);
                    let text = format!(" {} ({}) ", cat, count);
                    if i == state.current_category_index {
                        Span::styled(text, Style::default().bg(Color::Blue).fg(Color::White))
                    } else { Span::styled(text, Style::default().fg(Color::Gray)) }
                }).collect();
            let tabs = Paragraph::new(Line::from(category_tabs))
                .block(Block::default().borders(Borders::ALL).title("Categories [←/→]"));
            f.render_widget(tabs, chunks[1]);
            let current_category = state.current_category();
            let calcs = by_category.get(current_category).map(|v| v.as_slice()).unwrap_or(&[]);
            let items: Vec<ListItem> = calcs.iter().enumerate().map(|(i, calc)| {
                let style = if i == state.selected_calc_index {
                    Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
                } else { Style::default() };
                ListItem::new(vec![
                    Line::from(vec![Span::styled("🔹 ", Style::default().fg(Color::Yellow)),
                                    Span::styled(calc.name(), style.add_modifier(Modifier::BOLD))]),
                    Line::from(vec![Span::raw("   "),
                                    Span::styled(calc.description(), Style::default().fg(Color::Gray))]),
                    Line::from(vec![Span::raw("   ID: "),
                                    Span::styled(calc.id(), Style::default().fg(Color::Cyan))]),
                ]).style(style)
            }).collect();
            let list = List::new(items)
                .block(Block::default()
                    .title(format!("Calculators in {} [↑/↓]", current_category))
                    .borders(Borders::ALL));
            f.render_widget(list, chunks[2]);
            let help = Paragraph::new(vec![Line::from(vec![
                Span::styled("←/→", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(": Category  "),
                Span::styled("↑/↓", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(": Select  "),
                Span::styled("q", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw(": Quit"),
            ])]).block(Block::default().borders(Borders::ALL));
            f.render_widget(help, chunks[3]);
        })?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Left => state.prev_category(),
                KeyCode::Right => state.next_category(),
                KeyCode::Up => state.prev_calculator(),
                KeyCode::Down => {
                    let max = by_category.get(state.current_category())
                        .map(|v| v.len()).unwrap_or(0);
                    state.next_calculator(max);
                }
                _ => {}
            }
        }
    }
}