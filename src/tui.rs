use crate::app::{App, AppState};
use crate::board::{Cell, Grid};
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::io;
use std::time::Duration;

pub fn run(app: &mut App) -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    loop {
        terminal.draw(|frame| ui(frame, app))?;

        // If solving, step through the algorithm
        if app.state == AppState::Solving {
            app.step_solve();
            // Small delay to visualize steps
            std::thread::sleep(Duration::from_millis(10));
            
            // Check for user input to cancel
            if poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        handle_key_event(app, key);
                    }
                }
            }
        } else {
            // Normal event handling
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(app, key);
                }
            }
        }

        if app.should_quit {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.state {
        AppState::PuzzleSelection => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
            KeyCode::Down | KeyCode::Char('j') => app.next_puzzle(),
            KeyCode::Up | KeyCode::Char('k') => app.previous_puzzle(),
            KeyCode::Enter => {
                app.load_puzzle();
                app.solve_puzzle();
            }
            _ => {}
        },
        AppState::Solving => {
            // No interaction while solving
        }
        AppState::ViewingSolution => match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
            KeyCode::Char('b') | KeyCode::Backspace => app.back_to_selection(),
            _ => {}
        },
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    
    match app.state {
        AppState::PuzzleSelection => render_puzzle_selection(frame, chunks[1], app),
        AppState::Solving => render_solving(frame, chunks[1], app),
        AppState::ViewingSolution => render_solution(frame, chunks[1], app),
    }

    render_footer(frame, chunks[2], app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("FlowRS - Flow Free Puzzle Solver")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_puzzle_selection(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let items: Vec<ListItem> = app
        .puzzle_files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let content = if i == app.selected_puzzle_index {
                format!("> {}", file)
            } else {
                format!("  {}", file)
            };
            let style = if i == app.selected_puzzle_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title("Select Puzzle (↑/↓ or j/k)")
            .borders(Borders::ALL),
    );

    frame.render_widget(list, chunks[0]);

    let info_lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Recursive Backtracking Solver",
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Press Enter to solve"),
        Line::from("Press q or Esc to quit"),
    ];

    let info = Paragraph::new(info_lines)
        .block(Block::default().title("Solver Info").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(info, chunks[1]);
}

fn render_solving(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    // Status info
    let elapsed = app.solve_start_time.map(|start| start.elapsed()).unwrap_or_default();
    let steps = *app.steps_count.lock().unwrap();
    let status_text = format!(
        "Solving... | Steps: {} | Elapsed: {:.2}s",
        steps,
        elapsed.as_secs_f64()
    );
    
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().title("Status").borders(Borders::ALL));
    
    frame.render_widget(status, chunks[0]);

    // Show current solving grid
    let solving_grid_lock = app.solving_grid.lock().unwrap();
    if let Some(grid) = solving_grid_lock.as_ref() {
        render_grid(frame, chunks[1], grid);
    } else {
        let text = Paragraph::new("Initializing...")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(text, chunks[1]);
    }
}

fn render_solution(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    let info_text = if let (Some(duration), Some(result)) = (&app.solve_duration, &app.solve_result) {
        format!(
            "Puzzle: {} | Solver: Recursive Backtracking | Time: {:?} | Result: {}",
            app.puzzle_files[app.selected_puzzle_index],
            duration,
            result
        )
    } else {
        "No solution available".to_string()
    };

    let info = Paragraph::new(info_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(info, chunks[0]);

    if let Some(grid) = &app.solved_grid {
        render_grid(frame, chunks[1], grid);
    }
}

fn render_grid(frame: &mut Frame, area: Rect, grid: &Grid) {
    let mut lines = Vec::new();
    
    for row in &grid.cells {
        let mut spans = Vec::new();
        for cell in row {
            let (ch, color) = match cell {
                Cell::Empty => ('.', Color::DarkGray),
                Cell::Endpoint { colour } => ('O', cell_color(colour)),
                Cell::Path { colour } => ('o', cell_color(colour)),
            };
            spans.push(Span::styled(
                format!("{} ", ch),
                Style::default().fg(color),
            ));
        }
        lines.push(Line::from(spans));
    }

    let grid_widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default().title("Solution").borders(Borders::ALL));

    frame.render_widget(grid_widget, area);
}

fn cell_color(colour: &crate::board::Colour) -> Color {
    use crate::board::Colour;
    match colour {
        Colour::Red => Color::Red,
        Colour::Green => Color::Green,
        Colour::Blue => Color::Blue,
        Colour::Yellow => Color::Yellow,
        Colour::Magenta => Color::Magenta,
        Colour::Cyan => Color::Cyan,
        Colour::Orange => Color::Rgb(255, 165, 0),
        Colour::Brown => Color::Rgb(139, 69, 19),
        Colour::Purple => Color::Rgb(128, 0, 128),
        Colour::White => Color::White,
        Colour::Gray => Color::Gray,
        Colour::Lime => Color::Rgb(0, 255, 0),
        Colour::Beige => Color::Rgb(245, 245, 220),
        Colour::Navy => Color::Rgb(0, 0, 128),
        Colour::Teal => Color::Rgb(0, 128, 128),
        Colour::Pink => Color::Rgb(255, 192, 203),
    }
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.state {
        AppState::PuzzleSelection => {
            "↑/↓: Navigate | Enter: Solve | q/Esc: Quit"
        }
        AppState::Solving => "Solving in progress...",
        AppState::ViewingSolution => "b/Backspace: Back to Menu | q/Esc: Quit",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}
