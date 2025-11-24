use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;

use mazerion_core::{traits::get_all_calculators, CalcInput, Calculator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    List,
    Detail,
    Input,
}

struct App {
    mode: AppMode,
    selected: usize,
    calculators: Vec<Box<dyn Calculator>>,
    current_input: String,
    input_field: usize,
    inputs: Vec<(String, String)>,
    result: Option<String>,
    error: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            mode: AppMode::List,
            selected: 0,
            calculators: get_all_calculators(),
            current_input: String::new(),
            input_field: 0,
            inputs: Vec::new(),
            result: None,
            error: None,
        }
    }

    fn next(&mut self) {
        if self.calculators.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.calculators.len();
    }

    fn previous(&mut self) {
        if self.calculators.is_empty() {
            return;
        }
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.calculators.len() - 1;
        }
    }

    fn enter_detail(&mut self) {
        self.mode = AppMode::Detail;
        self.result = None;
        self.error = None;
        self.inputs.clear();
        self.current_input.clear();
        self.input_field = 0;
    }

    fn enter_input(&mut self) {
        self.mode = AppMode::Input;
        self.input_field = 0;
        self.inputs = self.get_required_inputs();
    }

    fn get_required_inputs(&self) -> Vec<(String, String)> {
        if self.selected >= self.calculators.len() {
            return Vec::new();
        }

        let calc_id = self.calculators[self.selected].id();

        // Define required inputs for each calculator
        match calc_id {
            "abv" => vec![("og".into(), String::new()), ("fg".into(), String::new())],
            "brix_to_sg" => vec![("brix".into(), String::new())],
            "sg_correction" => vec![("sg".into(), String::new()), ("temperature".into(), String::new())],
            "dilution" => vec![
                ("current_volume".into(), String::new()),
                ("current_abv".into(), String::new()),
                ("target_abv".into(), String::new()),
            ],
            "blending" => vec![
                ("volume1".into(), String::new()),
                ("abv1".into(), String::new()),
                ("volume2".into(), String::new()),
                ("abv2".into(), String::new()),
            ],
            "refractometer" => vec![
                ("original_brix".into(), String::new()),
                ("current_brix".into(), String::new()),
            ],
            "nutrition" => vec![
                ("volume".into(), String::new()),
                ("target_abv".into(), String::new()),
                ("yn_requirement".into(), "medium".into()),
            ],
            "carbonation" => vec![
                ("volume".into(), String::new()),
                ("temperature".into(), String::new()),
                ("target_co2".into(), String::new()),
                ("method".into(), "priming".into()),
            ],
            "sulfite" => vec![
                ("volume".into(), String::new()),
                ("ph".into(), String::new()),
                ("target_free_so2".into(), String::new()),
            ],
            "backsweetening" => vec![
                ("volume".into(), String::new()),
                ("current_sg".into(), String::new()),
                ("target_sg".into(), String::new()),
                ("sweetener".into(), "honey".into()),
            ],
            _ => vec![("value".into(), String::new())],
        }
    }

    fn next_field(&mut self) {
        if self.inputs.is_empty() {
            return;
        }
        self.input_field = (self.input_field + 1) % self.inputs.len();
        self.current_input = self.inputs[self.input_field].1.clone();
    }

    fn prev_field(&mut self) {
        if self.inputs.is_empty() {
            return;
        }
        if self.input_field > 0 {
            self.input_field -= 1;
        } else {
            self.input_field = self.inputs.len() - 1;
        }
        self.current_input = self.inputs[self.input_field].1.clone();
    }

    fn save_current_field(&mut self) {
        if self.input_field < self.inputs.len() {
            self.inputs[self.input_field].1 = self.current_input.clone();
        }
    }

    fn calculate(&mut self) {
        if self.selected >= self.calculators.len() {
            self.error = Some("No calculator selected".into());
            return;
        }

        let calc = &self.calculators[self.selected];
        let mut input = CalcInput::new();

        for (key, value) in &self.inputs {
            input = input.add_param(key, value);
        }

        match calc.calculate(input) {
            Ok(result) => {
                self.result = Some(format!(
                    "✓ Result: {} {}\n\nMetadata:\n{}",
                    result.output.value,
                    result.output.unit,
                    result.metadata.iter()
                        .map(|(k, v)| format!("• {}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join("\n")
                ));
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Error: {}", e));
                self.result = None;
            }
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

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Title
            let title = Paragraph::new("🍯 Mazerion TUI - 39 Professional Calculators")
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            f.render_widget(title, chunks[0]);

            // Main content
            match app.mode {
                AppMode::List => render_list(f, chunks[1], app),
                AppMode::Detail => render_detail(f, chunks[1], app),
                AppMode::Input => render_input(f, chunks[1], app),
            }

            // Status bar
            let status_text = match app.mode {
                AppMode::List => "↑/↓: Navigate | Enter: Select | q: Quit",
                AppMode::Detail => "Enter: Input Values | Backspace: Back | q: Quit",
                AppMode::Input => "Tab: Next Field | Enter: Calculate | Esc: Back | q: Quit",
            };
            let status_bar = Paragraph::new(status_text)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(status_bar, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                AppMode::List => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.enter_detail(),
                    _ => {}
                },
                AppMode::Detail => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Backspace => app.mode = AppMode::List,
                    KeyCode::Enter => app.enter_input(),
                    _ => {}
                },
                AppMode::Input => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Esc => app.mode = AppMode::Detail,
                    KeyCode::Tab => {
                        app.save_current_field();
                        app.next_field();
                    }
                    KeyCode::BackTab => {
                        app.save_current_field();
                        app.prev_field();
                    }
                    KeyCode::Enter => {
                        app.save_current_field();
                        app.calculate();
                    }
                    KeyCode::Char(c) => {
                        app.current_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.current_input.pop();
                    }
                    _ => {}
                },
            }
        }
    }
}

fn render_list(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .calculators
        .iter()
        .enumerate()
        .map(|(i, calc)| {
            let style = if i == app.selected {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(vec![Line::from(vec![
                Span::styled(
                    format!("{:20}", calc.name()),
                    style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(" - ", style),
                Span::styled(calc.description(), style.fg(Color::Gray)),
            ])])
                .style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(format!("Calculators ({}/{})", app.selected + 1, app.calculators.len()))
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_widget(list, area);
}

fn render_detail(f: &mut ratatui::Frame, area: Rect, app: &App) {
    if app.selected >= app.calculators.len() {
        return;
    }

    let calc = &app.calculators[app.selected];

    let text = vec![
        Line::from(vec![
            Span::styled("Calculator: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(calc.name()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(calc.category()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Description: ", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(calc.description()),
        Line::from(""),
        Line::from(vec![
            Span::styled("Help: ", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(calc.help_text()),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press Enter to input values and calculate", Style::default().fg(Color::Green)),
        ]),
    ];

    if let Some(ref result) = app.result {
        let mut result_text = text.clone();
        result_text.push(Line::from(""));
        result_text.push(Line::from(vec![
            Span::styled("Last Result:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
        ]));
        for line in result.lines() {
            result_text.push(Line::from(Span::styled(line, Style::default().fg(Color::Cyan))));
        }
        let paragraph = Paragraph::new(result_text)
            .block(Block::default().title("Calculator Details").borders(Borders::ALL))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(paragraph, area);
    } else {
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Calculator Details").borders(Borders::ALL))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(paragraph, area);
    }
}

fn render_input(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut text = vec![
        Line::from(vec![
            Span::styled("Input Values", Style::default().add_modifier(Modifier::BOLD).fg(Color::Cyan)),
        ]),
        Line::from(""),
    ];

    for (i, (key, value)) in app.inputs.iter().enumerate() {
        let style = if i == app.input_field {
            Style::default().bg(Color::Blue).fg(Color::White)
        } else {
            Style::default()
        };

        let display_value = if i == app.input_field {
            &app.current_input
        } else {
            value
        };

        text.push(Line::from(vec![
            Span::styled(format!("{}: ", key), style.add_modifier(Modifier::BOLD)),
            Span::styled(display_value, style),
        ]));
    }

    text.push(Line::from(""));

    if let Some(ref result) = app.result {
        text.push(Line::from(vec![
            Span::styled("Result:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green)),
        ]));
        for line in result.lines() {
            text.push(Line::from(Span::styled(line, Style::default().fg(Color::Green))));
        }
    }

    if let Some(ref error) = app.error {
        text.push(Line::from(vec![
            Span::styled("Error:", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
        ]));
        text.push(Line::from(Span::styled(error, Style::default().fg(Color::Red))));
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Input Mode").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(paragraph, area);
}