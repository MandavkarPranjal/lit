use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

pub fn run_tui() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    terminal::enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout.lock());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = ListState::default();
    let options = vec![
        "Add Profile",
        "Switch Profile",
        "Update Profile",
        "Delete Profile",
        "Exit",
    ];

    state.select(Some(0)); // Initialize with the first option selected
    let mut last_event_time = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(size);

            let items: Vec<ListItem> = options
                .iter()
                .map(|o| ListItem::new(o.to_string()))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Menu"))
                .highlight_style(tui::style::Style::default().bg(tui::style::Color::Yellow));

            f.render_stateful_widget(list, chunks[0], &mut state);

            let paragraph =
                Paragraph::new("Use arrow keys or 'j', 'k' to navigate and Enter to select.")
                    .block(Block::default().title("Instructions").borders(Borders::ALL));
            f.render_widget(paragraph, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                if last_event_time.elapsed() < Duration::from_millis(200) {
                    continue; // Skip the event if it's too soon after the last one
                }
                last_event_time = Instant::now();

                match code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        let i = state.selected().unwrap_or(0);
                        if i > 0 {
                            state.select(Some(i - 1));
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let i = state.selected().unwrap_or(0);
                        if i < options.len() - 1 {
                            state.select(Some(i + 1));
                        }
                    }
                    KeyCode::Enter => match state.selected() {
                        Some(0) => add_profile(),
                        Some(1) => switch_profile(),
                        Some(2) => update_profile(),
                        Some(3) => delete_profile(),
                        Some(4) => break,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn add_profile() {
    println!("Add Profile functionality.");
    // Implement functionality to add a profile.
}

fn switch_profile() {
    println!("Switch Profile functionality.");
    // Implement functionality to switch profiles.
}

fn update_profile() {
    println!("Update Profile functionality.");
    // Implement functionality to update a profile.
}

fn delete_profile() {
    println!("Delete Profile functionality.");
    // Implement functionality to delete a profile.
}
