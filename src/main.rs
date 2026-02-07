mod libdatabend;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap},
};
use std::{
    io::{self, Stdout},
    path::Path,
    time::{Duration, Instant},
};

type Tui = Terminal<CrosstermBackend<Stdout>>;

#[derive(Debug, Clone)]
struct DatabendEffect {
    name: String,
    description: String,
    emoji: String,
    params: Vec<String>,
}

struct App {
    effects: Vec<DatabendEffect>,
    selected_effect: usize,
    input_path: String,
    output_path: String,
    params: Vec<String>,
    current_input: InputMode,
    processing: bool,
    progress: f64,
    status_message: String,
    last_update: Instant,
    synesthesia_state: Option<libdatabend::synestesia::SynesthesiaState>,
}

#[derive(Debug, PartialEq)]
enum InputMode {
    SelectingEffect,
    InputPath,
    OutputPath,
    Parameters(usize),
    Processing,
}

impl App {
    fn new() -> Self {
        let effects = vec![
            DatabendEffect {
                name: "Oversensibility".to_string(),
                description: "Simulates high ISO sensitivity with random noise corruption"
                    .to_string(),
                emoji: "📸".to_string(),
                params: vec!["ISO (0-6400)".to_string()],
            },
            DatabendEffect {
                name: "Overexposure".to_string(),
                description: "Creates blown-out highlights with random brightness boosts"
                    .to_string(),
                emoji: "☀️".to_string(),
                params: vec!["Exposure Factor (0.1-3.0)".to_string()],
            },
            DatabendEffect {
                name: "Synesthesia".to_string(),
                description: "Interactive databending - press keys to bend reality".to_string(),
                emoji: "🎹".to_string(),
                params: vec![],
            },
            DatabendEffect {
                name: "Variations on a Cloud".to_string(),
                description: "Creates glitchy patches by copying random image regions".to_string(),
                emoji: "☁️".to_string(),
                params: vec!["Patch Size (10-200)".to_string()],
            },
            DatabendEffect {
                name: "The Mind Electric".to_string(),
                description: "Layered chaos with alpha blending and color shifts".to_string(),
                emoji: "⚡".to_string(),
                params: vec!["Layers (1-20)".to_string()],
            },
            DatabendEffect {
                name: "Jack Stauberism".to_string(),
                description: "Lyrical databending with song lyrics as corruption data".to_string(),
                emoji: "🎵".to_string(),
                params: vec![],
            },
            DatabendEffect {
                name: "New Normal".to_string(),
                description: "Interactive chaos mode - embrace the new normal".to_string(),
                emoji: "🌈".to_string(),
                params: vec![],
            },
        ];

        Self {
            effects,
            selected_effect: 0,
            input_path: String::new(),
            output_path: String::new(),
            params: vec![String::new(); 1],
            current_input: InputMode::SelectingEffect,
            processing: false,
            progress: 0.0,
            status_message: "Ready — select an effect and set file paths to begin".to_string(),
            last_update: Instant::now(),
            synesthesia_state: None,
        }
    }

    fn next_effect(&mut self) {
        self.selected_effect = (self.selected_effect + 1) % self.effects.len();
        self.update_params();
    }

    fn previous_effect(&mut self) {
        if self.selected_effect > 0 {
            self.selected_effect -= 1;
        } else {
            self.selected_effect = self.effects.len() - 1;
        }
        self.update_params();
    }

    fn update_params(&mut self) {
        let param_count = self.effects[self.selected_effect].params.len();
        self.params = vec![String::new(); param_count.max(1)];
    }

    fn execute_effect(&mut self) {
        if self.input_path.is_empty() || self.output_path.is_empty() {
            self.status_message = "❌ please specify input and output paths!".to_string();
            return;
        }

        if !Path::new(&self.input_path).exists() {
            self.status_message = "❌ input file does not exist!".to_string();
            return;
        }

        self.processing = true;
        self.progress = 0.0;
        self.status_message = " currently processing...".to_string();

        // Execute the selected effect
        match self.selected_effect {
            0 => {
                // Oversensibility
                let iso = self.params[0].parse::<i32>().unwrap_or(800);
                libdatabend::oversensibility::main(&self.input_path, &self.output_path, &iso);
            }
            1 => {
                // Overexposure
                let exposure = self.params[0].parse::<f32>().unwrap_or(1.5);
                libdatabend::overexposure::main(&self.input_path, &self.output_path, exposure);
            }
            2 => {
                // Synesthesia - Initialize interactive mode
                match libdatabend::synestesia::SynesthesiaState::new(&self.input_path) {
                    Ok(state) => {
                        self.synesthesia_state = Some(state);
                        self.current_input = InputMode::Processing;
                        self.status_message = "🎹 Synesthesia mode active! Press keys to databend, ESC to finish!".to_string();
                    }
                    Err(error) => {
                        self.status_message = format!("❌ Failed to start synesthesia: {}", error);
                    }
                }
                return; // Don't set processing to false
            }
            3 => {
                // Variations on a Cloud
                let layers = self.params[0].parse::<u32>().unwrap_or(5);
                let _ = libdatabend::variationsonacloud::main(
                    &self.input_path,
                    &self.output_path,
                    layers,
                );
            }
            4 => {
                // The Mind Electric
                let layers = self.params[0].parse::<u32>().unwrap_or(5);
                let _ = libdatabend::themindelectric::main(
                    &self.input_path,
                    &self.output_path,
                    &layers,
                );
            }
            5 => {
                // Jack Stauberism
                let _ = libdatabend::jackstauberism::main(&self.input_path, &self.output_path);
            }
            6 => {
                // New Normal
                libdatabend::newnormal::main(&self.input_path, &self.output_path);
            }
            _ => {}
        }

        self.processing = false;
        self.progress = 100.0;
        self.status_message = "✅ Effect applied successfully!".to_string();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app(terminal: &mut Tui, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Ok(true) = event::poll(Duration::from_millis(50)) {
            app.last_update = Instant::now();
            if let Event::Key(key) = event::read()? {
                // Only process KeyEventKind::Press to avoid duplicates from key repeat
                if key.kind == KeyEventKind::Press {
                    match app.current_input {
                        InputMode::SelectingEffect => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Up | KeyCode::Char('k') => app.previous_effect(),
                            KeyCode::Down | KeyCode::Char('j') => app.next_effect(),
                            KeyCode::Char('i') => app.current_input = InputMode::InputPath,
                            KeyCode::Char('o') => app.current_input = InputMode::OutputPath,
                            KeyCode::Char('p') => {
                                if !app.effects[app.selected_effect].params.is_empty() {
                                    app.current_input = InputMode::Parameters(0);
                                }
                            }
                            KeyCode::Enter => app.execute_effect(),
                            _ => {}
                        },
                        InputMode::InputPath => match key.code {
                            KeyCode::Enter => app.current_input = InputMode::SelectingEffect,
                            KeyCode::Esc => app.current_input = InputMode::SelectingEffect,
                            KeyCode::Backspace => {
                                app.input_path.pop();
                            }
                            KeyCode::Char(c) => app.input_path.push(c),
                            _ => {}
                        },
                        InputMode::OutputPath => match key.code {
                            KeyCode::Enter => app.current_input = InputMode::SelectingEffect,
                            KeyCode::Esc => app.current_input = InputMode::SelectingEffect,
                            KeyCode::Backspace => {
                                app.output_path.pop();
                            }
                            KeyCode::Char(c) => app.output_path.push(c),
                            _ => {}
                        },
                        InputMode::Parameters(idx) => match key.code {
                            KeyCode::Enter => {
                                app.execute_effect();
                                app.current_input = InputMode::Processing;
                                app.processing = true;
                                app.progress = 0.0;
                            }

                            KeyCode::Esc => app.current_input = InputMode::SelectingEffect,
                            KeyCode::Backspace => {
                                app.params[idx].pop();
                            }
                            KeyCode::Char(c) => app.params[idx].push(c),
                            _ => {}
                        },
                        InputMode::Processing => {
                            if app.synesthesia_state.is_some() {
                                // Special handling for synesthesia mode
                                match key.code {
                                    KeyCode::Esc => {
                                        // Save and exit synesthesia mode
                                        if let Some(state) = &app.synesthesia_state {
                                            match state.save(&app.output_path) {
                                                Ok(message) => app.status_message = message,
                                                Err(error) => app.status_message = format!("❌ {}", error),
                                            }
                                        }
                                        app.synesthesia_state = None;
                                        app.current_input = InputMode::SelectingEffect;
                                        app.processing = false;
                                    }
                                    KeyCode::Char(c) => {
                                        // Process the key press in synesthesia mode
                                        if let Some(state) = &mut app.synesthesia_state {
                                            app.status_message = state.process_key(c);
                                        }
                                    }
                                    _ => {}
                                }
                            } else {
                                // Normal processing mode
                                if key.code == KeyCode::Esc {
                                    app.current_input = InputMode::SelectingEffect;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update progress animation
        if app.processing {
            app.progress = (app.progress + 2.0).min(100.0);
        }
    }
}

fn mode_label(mode: &InputMode) -> &'static str {
    match mode {
        InputMode::SelectingEffect => "Select Effect",
        InputMode::InputPath => "Editing Input Path",
        InputMode::OutputPath => "Editing Output Path",
        InputMode::Parameters(_) => "Editing Parameters",
        InputMode::Processing => "Processing",
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Main content
            Constraint::Length(4), // Status
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📸💣 Shutterbomb v0.2 — Interactive Image Databending")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .border_style(Style::default().fg(Color::Magenta)),
        );
    f.render_widget(title, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // Effects list
    let effects: Vec<ListItem> = app
        .effects
        .iter()
        .enumerate()
        .map(|(i, effect)| {
            let style = if i == app.selected_effect {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::raw(format!("{} ", &effect.emoji)),
                Span::styled(&*effect.name, style),
            ]))
            .style(style)
        })
        .collect();

    let effects_border = if app.current_input == InputMode::SelectingEffect {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    let effects_list = List::new(effects)
        .block(
            Block::default()
                .title("Effects")
                .borders(Borders::ALL)
                .border_style(effects_border),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(effects_list, main_chunks[0]);

    // Right panel
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4), // Description
            Constraint::Length(3), // Input path
            Constraint::Length(3), // Output path
            Constraint::Min(3),    // Parameters
        ])
        .split(main_chunks[1]);

    // Description
    let selected = &app.effects[app.selected_effect];
    let desc = Paragraph::new(format!("{} {}", selected.emoji, selected.description))
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("Description")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(desc, right_chunks[0]);

    // Input path
    let input_active = app.current_input == InputMode::InputPath;
    let input_style = if input_active {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let input_content = if app.input_path.is_empty() && !input_active {
        Span::styled("(none)", Style::default().fg(Color::DarkGray))
    } else if input_active {
        Span::styled(format!("{}▏", &app.input_path), input_style)
    } else {
        Span::styled(&*app.input_path, input_style)
    };
    let input = Paragraph::new(Line::from(input_content))
        .block(
            Block::default()
                .title("Input Path [i]")
                .borders(Borders::ALL)
                .border_style(input_style),
        );
    f.render_widget(input, right_chunks[1]);

    // Output path
    let output_active = app.current_input == InputMode::OutputPath;
    let output_style = if output_active {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let output_content = if app.output_path.is_empty() && !output_active {
        Span::styled("(none)", Style::default().fg(Color::DarkGray))
    } else if output_active {
        Span::styled(format!("{}▏", &app.output_path), output_style)
    } else {
        Span::styled(&*app.output_path, output_style)
    };
    let output = Paragraph::new(Line::from(output_content))
        .block(
            Block::default()
                .title("Output Path [o]")
                .borders(Borders::ALL)
                .border_style(output_style),
        );
    f.render_widget(output, right_chunks[2]);

    // Parameters
    let param_border = if matches!(app.current_input, InputMode::Parameters(_)) {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Cyan)
    };

    if !selected.params.is_empty() {
        let param_text = selected
            .params
            .iter()
            .zip(&app.params)
            .enumerate()
            .map(|(i, (param_name, value))| {
                let editing = matches!(app.current_input, InputMode::Parameters(idx) if idx == i);
                let style = if editing {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                let display = if editing {
                    format!("{}▏", value)
                } else if value.is_empty() {
                    "(default)".to_string()
                } else {
                    value.clone()
                };
                Line::from(vec![
                    Span::styled(
                        format!("{}: ", param_name),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(display, style),
                ])
            })
            .collect::<Vec<_>>();

        let params = Paragraph::new(Text::from(param_text)).block(
            Block::default()
                .title("Parameters [p]")
                .borders(Borders::ALL)
                .border_style(param_border),
        );
        f.render_widget(params, right_chunks[3]);
    } else {
        let no_params = Paragraph::new(
            Span::styled("This effect has no configurable parameters.", Style::default().fg(Color::DarkGray))
        ).block(
            Block::default()
                .title("Parameters")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(no_params, right_chunks[3]);
    }

    // Status bar — context-sensitive help per mode
    let controls_line = match &app.current_input {
        InputMode::SelectingEffect => Line::from(vec![
            Span::styled("↑↓/j/k", Style::default().fg(Color::Cyan)),
            Span::raw(" select  "),
            Span::styled("i", Style::default().fg(Color::Cyan)),
            Span::raw(" input path  "),
            Span::styled("o", Style::default().fg(Color::Cyan)),
            Span::raw(" output path  "),
            Span::styled("p", Style::default().fg(Color::Cyan)),
            Span::raw(" params  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" run  "),
            Span::styled("q/Esc", Style::default().fg(Color::Red)),
            Span::raw(" quit"),
        ]),
        InputMode::InputPath | InputMode::OutputPath => Line::from(vec![
            Span::raw("Type a file path, then press "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" to confirm or "),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(" to cancel"),
        ]),
        InputMode::Parameters(_) => Line::from(vec![
            Span::raw("Type a value, then press "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" to run or "),
            Span::styled("Esc", Style::default().fg(Color::Red)),
            Span::raw(" to cancel"),
        ]),
        InputMode::Processing => {
            if app.synesthesia_state.is_some() {
                Line::from(vec![
                    Span::raw("Press keys to databend the image. "),
                    Span::styled("Esc", Style::default().fg(Color::Red)),
                    Span::raw(" to save and exit"),
                ])
            } else {
                Line::from(vec![
                    Span::styled("Esc", Style::default().fg(Color::Red)),
                    Span::raw(" to return"),
                ])
            }
        }
    };

    let status_text = vec![
        Line::from(vec![
            Span::styled("Mode: ", Style::default().fg(Color::White)),
            Span::styled(
                mode_label(&app.current_input),
                Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(&*app.status_message, Style::default().fg(Color::Green)),
        ]),
        controls_line,
    ];

    let status = Paragraph::new(Text::from(status_text)).block(
        Block::default()
            .title("Status")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White)),
    );
    f.render_widget(status, chunks[2]);

    // Processing overlay
    if app.processing {
        let area = centered_rect(50, 20, f.area());
        f.render_widget(Clear, area);
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title("Processing...")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .gauge_style(Style::default().fg(Color::Green))
            .percent(app.progress as u16);
        f.render_widget(gauge, area);
    }
}

fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
