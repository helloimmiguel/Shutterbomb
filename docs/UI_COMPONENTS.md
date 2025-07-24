# 🖥️ User Interface Components Documentation

Detailed documentation of the TUI components and interface design.

## 📋 Table of Contents

1. [Layout Architecture](#layout-architecture)
2. [Component Hierarchy](#component-hierarchy)
3. [Styling System](#styling-system)
4. [Input Handling](#input-handling)
5. [State Management](#state-management)

## Layout Architecture

### Main Layout Structure

```
┌─────────────────────────────────────────┐
│              Title Bar (3)              │ ← Fixed height
├─────────────────┬───────────────────────┤
│                 │                       │
│   Effects List  │    Right Panel        │ ← Expandable area
│      (50%)      │       (50%)           │
│                 │                       │
├─────────────────┴───────────────────────┤
│             Status Bar (3)              │ ← Fixed height
└─────────────────────────────────────────┘
```

### Constraint System

```rust
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),    // Title: Fixed 3 lines
        Constraint::Min(10),      // Main: Minimum 10 lines, expands
        Constraint::Length(3),    // Status: Fixed 3 lines
    ])
    .split(f.area());
```

## Component Hierarchy

### 1. Title Bar Component

**Location**: `chunks[0]`
**Type**: `Paragraph`

```rust
let title = Paragraph::new("📸💣 |Shutterbomb - v0.1 - 🎵 I've began to databend 🎵| 📸💣")
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Magenta)),
    );
```

**Features**:
- **Double Border**: `BorderType::Double` for emphasis
- **Center Alignment**: Title centered horizontally
- **Cyan Text**: High visibility on dark backgrounds
- **Bold Modifier**: Enhanced readability
- **Magenta Border**: Distinctive framing

---

### 2. Main Content Area

**Layout**: Horizontal split (50/50)

#### 2.1 Effects List (Left Panel)

**Location**: `main_chunks[0]`
**Type**: `List<ListItem>`

```rust
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
```

**Dynamic Elements**:
- **Selection Highlighting**: Blue background for selected item
- **Emoji Icons**: Visual identification for each effect
- **Style Inheritance**: Item-level styling propagation

**Data Binding**:
- **Effect Names**: From `DatabendEffect.name`
- **Emojis**: From `DatabendEffect.emoji`
- **Selection State**: From `App.selected_effect`

#### 2.2 Right Panel (Information Display)

**Layout**: Vertical stack with dynamic constraints

```rust
let right_chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(4),    // Description: Fixed height
        Constraint::Length(3),    // Input path: Fixed height
        Constraint::Length(3),    // Output path: Fixed height
        Constraint::Min(3),       // Parameters: Expandable
    ])
    .split(main_chunks[1]);
```

##### 2.2.1 Description Panel

**Location**: `right_chunks[0]`
**Type**: `Paragraph` with wrapping

```rust
let desc = Paragraph::new(app.effects[app.selected_effect].description.clone())
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .title("📝 Description")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    )
    .wrap(Wrap { trim: true });
```

**Features**:
- **Text Wrapping**: `Wrap { trim: true }` for long descriptions
- **Dynamic Content**: Updates based on selected effect
- **Yellow Styling**: Consistent with informational content
- **Icon Title**: 📝 for visual identification

##### 2.2.2 Input Path Panel

**Location**: `right_chunks[1]`
**Type**: `Paragraph` with conditional styling

```rust
let input_style = if app.current_input == InputMode::InputPath {
    Style::default().fg(Color::Green)
} else {
    Style::default()
};
```

**State-Dependent Features**:
- **Active Highlighting**: Green color when editing
- **Visual Feedback**: Clear indication of current input mode
- **Help Text**: Title includes keyboard shortcut "(press 'i')"

##### 2.2.3 Output Path Panel

**Location**: `right_chunks[2]`
**Type**: `Paragraph` with conditional styling

Similar to input path but for output destination.

##### 2.2.4 Parameters Panel

**Location**: `right_chunks[3]`
**Type**: `Paragraph` with dynamic content

```rust
if !app.effects[app.selected_effect].params.is_empty() {
    let param_text = app.effects[app.selected_effect].params
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
                Span::styled(format!("{}: ", param_name), Style::default().fg(Color::Cyan)),
                Span::styled(value.clone(), style),
            ])
        })
        .collect::<Vec<_>>();
```

**Advanced Features**:
- **Conditional Rendering**: Only shown when effect has parameters
- **Parameter Iteration**: Combines parameter names with current values
- **Individual Highlighting**: Each parameter can be independently selected
- **Complex Matching**: Uses `matches!` macro for pattern matching

---

### 3. Status Bar Component

**Location**: `chunks[2]`
**Type**: `Paragraph` with multi-line content

```rust
let status_text = vec![
    Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::White)),
        Span::styled(app.status_message.clone(), Style::default().fg(Color::Green)),
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
```

**Information Architecture**:
- **Line 1**: Current application status
- **Line 2**: Available keyboard controls

**Styling Strategy**:
- **White Labels**: "Status:", "Controls:"
- **Green Status**: Current status message
- **Cyan Actions**: Available commands
- **Red Exit**: Quit commands for emphasis

---

### 4. Processing Overlay

**Type**: Modal overlay with progress indicator
**Condition**: Only rendered when `app.processing == true`

```rust
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
```

**Overlay Implementation**:
- **Centered Positioning**: `centered_rect()` utility function
- **Clear Background**: `Clear` widget removes underlying content
- **Progress Visualization**: `Gauge` widget shows completion percentage
- **Modal Behavior**: Blocks interaction with underlying interface

#### Centered Rectangle Utility

```rust
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
```

## Styling System

### Color Palette

| Component | Primary | Secondary | Accent |
|-----------|---------|-----------|--------|
| Title | Cyan | Magenta (border) | - |
| Effects List | Blue (selection) | Green (border) | White (text) |
| Description | Yellow | Yellow (border) | - |
| Input Fields | Green (active) | Default (inactive) | - |
| Parameters | Cyan (labels) | Green (active) | - |
| Status | Green (message) | White (labels) | - |
| Controls | Cyan (commands) | Red (quit) | - |
| Processing | Green | Green (border) | - |

### Style Modifiers

```rust
// Bold text for emphasis
.add_modifier(Modifier::BOLD)

// Background highlighting
.bg(Color::Blue)

// Foreground coloring
.fg(Color::Cyan)

// Combined styling
Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
```

### Border Styles

```rust
// Standard border
.borders(Borders::ALL)

// Double border for title
.border_type(BorderType::Double)

// Colored borders
.border_style(Style::default().fg(Color::Magenta))
```

## Input Handling

### Event Processing Pipeline

```rust
if let Ok(true) = event::poll(Duration::from_millis(50)) {
    if let Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press {
            match app.current_input {
                // Handle different input modes
            }
        }
    }
}
```

### Input Mode State Machine

```
SelectingEffect ──i──> InputPath
       │              │
       │              └──Enter/Esc──> SelectingEffect
       │
       ├──o──> OutputPath
       │              │
       │              └──Enter/Esc──> SelectingEffect
       │
       ├──p──> Parameters(0)
       │              │
       │              └──Enter/Esc──> SelectingEffect
       │
       └──Enter──> Processing ──automatic──> SelectingEffect
```

### Key Bindings by Mode

#### SelectingEffect Mode
- `↑`/`k`: Previous effect
- `↓`/`j`: Next effect
- `i`: Enter input path mode
- `o`: Enter output path mode
- `p`: Enter parameters mode (if available)
- `Enter`: Execute effect
- `q`/`Esc`: Quit application

#### Input/Output Path Modes
- `Char(c)`: Append character
- `Backspace`: Remove last character
- `Enter`/`Esc`: Return to effect selection

#### Parameters Mode
- `Char(c)`: Append to current parameter
- `Backspace`: Remove from current parameter
- `Enter`/`Esc`: Return to effect selection

## State Management

### Application State Structure

```rust
struct App {
    // Core data
    effects: Vec<DatabendEffect>,     // Static effect definitions
    selected_effect: usize,           // Current selection index
    
    // User input
    input_path: String,               // Source file path
    output_path: String,              // Destination file path
    params: Vec<String>,              // Parameter values
    
    // UI state
    current_input: InputMode,         // Current input focus
    
    // Processing state
    processing: bool,                 // Processing active flag
    progress: f64,                    // Completion percentage
    status_message: String,           // User feedback
    
    // Timing
    last_update: Instant,             // Last state change
}
```

### State Transitions

#### Effect Selection
```rust
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
```

#### Parameter Management
```rust
fn update_params(&mut self) {
    let param_count = self.effects[self.selected_effect].params.len();
    self.params = vec![String::new(); param_count.max(1)];
}
```

### UI Refresh Strategy

- **Continuous Polling**: 50ms intervals for responsive input
- **Full Redraws**: Complete UI refresh on every frame
- **State-Driven Rendering**: UI reflects current application state
- **Conditional Elements**: Components shown/hidden based on state

### Performance Considerations

- **String Cloning**: Effect names and descriptions cloned for rendering
- **Vector Operations**: Effects list rebuilt every frame
- **Memory Allocation**: Parameter vectors reallocated on effect change
- **Terminal Bandwidth**: Full screen refresh each cycle

---

*Complete UI component reference for Shutterbomb*