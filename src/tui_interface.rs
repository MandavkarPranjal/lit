use crate::config::{load_config, save_config, GitConfig};
use crate::input::{handle_input, InputMode};
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};
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
        // "Exit",
    ];

    state.select(Some(0)); // Initialize with the first option selected
    let mut last_event_time = Instant::now();
    let mut input_mode = InputMode::Normal;
    let mut profile_name = String::new();
    let mut user_name = String::new();
    let mut user_email = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(size);

            match input_mode {
                InputMode::InputProfileName => {
                    let paragraph = Paragraph::new(format!("Profile Name: {}", profile_name))
                        .block(
                            Block::default()
                                .title("Enter Profile Name")
                                .borders(Borders::ALL),
                        );
                    f.render_widget(paragraph, chunks[1]);
                }
                InputMode::InputUserName => {
                    let paragraph = Paragraph::new(format!("User Name: {}", user_name)).block(
                        Block::default()
                            .title("Enter User Name")
                            .borders(Borders::ALL),
                    );
                    f.render_widget(paragraph, chunks[1]);
                }
                InputMode::InputUserEmail => {
                    let paragraph = Paragraph::new(format!("User Email: {}", user_email)).block(
                        Block::default()
                            .title("Enter User Email")
                            .borders(Borders::ALL),
                    );
                    f.render_widget(paragraph, chunks[1]);
                }
                InputMode::Normal => {
                    let items: Vec<ListItem> = options
                        .iter()
                        .map(|o| ListItem::new(o.to_string()))
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Menu"))
                        .highlight_style(
                            tui::style::Style::default().bg(tui::style::Color::Yellow),
                        );

                    f.render_stateful_widget(list, chunks[0], &mut state);

                    let paragraph = Paragraph::new(
                        "Use arrow keys or 'j', 'k' to navigate, 'Enter' to select, 'q' to exit.",
                    )
                    .block(Block::default().title("Instructions").borders(Borders::ALL));
                    f.render_widget(paragraph, chunks[1]);
                }
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if last_event_time.elapsed() < Duration::from_millis(200) {
                    continue; // Skip the event if it's too soon after the last one
                }
                last_event_time = Instant::now();

                if key.code == KeyCode::Char('q') {
                    break;
                }

                if let Err(err) = handle_input(
                    key,
                    &mut input_mode,
                    &mut state,
                    &options,
                    &mut profile_name,
                    &mut user_name,
                    &mut user_email,
                ) {
                    eprintln!("Error handling input: {:?}", err);
                    break;
                }

                // if input_mode == InputMode::Normal && state.selected() == Some(options.len() - 1) {
                //     break;
                // }
            }
        }
    }
    Ok(())
}

// fn add_profile(profile_name: &str, user_name: &str, user_email: &str) -> io::Result<()> {
//     let mut config = load_config();
//     config.profiles.insert(
//         profile_name.to_string(),
//         GitConfig {
//             user_name: user_name.to_string(),
//             user_email: user_email.to_string(),
//         },
//     );
//     save_config(&config);
//     Ok(())
// }

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
