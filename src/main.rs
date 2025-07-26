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
            status_message: "welcome to shutterbomb!! 📸💣".to_string(),
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
        self.status_message = "you've databent successfully!".to_string();
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

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Status
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("📸💣 |Shutterbomb - v0.2 - 🎵 I've began to databend 🎵| 📸💣")
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
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
                Span::raw(effect.emoji.clone()),
                Span::raw(" "),
                Span::styled(effect.name.clone(), style),
            ]))
            .style(style)
        })
        .collect();

    let effects_list = List::new(effects)
        .block(
            Block::default()
                .title("📋 Effects")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
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
    let desc = Paragraph::new(app.effects[app.selected_effect].description.clone())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .title("📝 Description")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(desc, right_chunks[0]);

    // Input path
    let input_style = if app.current_input == InputMode::InputPath {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let input = Paragraph::new(app.input_path.clone())
        .style(input_style)
        .block(
            Block::default()
                .title("📁 Input Path (press 'i')")
                .borders(Borders::ALL)
                .border_style(input_style),
        );
    f.render_widget(input, right_chunks[1]);

    // Output path
    let output_style = if app.current_input == InputMode::OutputPath {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };
    let output = Paragraph::new(app.output_path.clone())
        .style(output_style)
        .block(
            Block::default()
                .title("💾 Output Path (press 'o')")
                .borders(Borders::ALL)
                .border_style(output_style),
        );
    f.render_widget(output, right_chunks[2]);

    // Parameters
    if !app.effects[app.selected_effect].params.is_empty() {
        let param_text = app.effects[app.selected_effect]
            .params
            .iter()
            .zip(&app.params)
            .enumerate()
            .map(|(i, (param_name, value))| {
                let style = if matches!(app.current_input, InputMode::Parameters(idx) if idx == i) {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                Line::from(vec![
                    Span::styled(
                        format!("{}: ", param_name),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(value.clone(), style),
                ])
            })
            .collect::<Vec<_>>();

        let params = Paragraph::new(Text::from(param_text)).block(
            Block::default()
                .title("⚙️ Parameters (p)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
        f.render_widget(params, right_chunks[3]);
    }

    // Status bar
    let status_text = vec![
        Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::White)),
            Span::styled(
                app.status_message.clone(),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Controls: "),
            Span::styled("↑↓", Style::default().fg(Color::Cyan)),
            Span::raw(" select, "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" execute, "),
            Span::styled("i/o/p", Style::default().fg(Color::Cyan)),
            Span::raw(" edit, "),
            Span::styled("q/Esc", Style::default().fg(Color::Red)),
            Span::raw(" quit"),
        ]),
    ];

    let status = Paragraph::new(Text::from(status_text)).block(
        Block::default()
            .title("📊 Status")
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
                    .title("🚀 Processing...")
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
