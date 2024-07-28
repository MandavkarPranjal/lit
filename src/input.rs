use crossterm::event::{KeyCode, KeyEvent};
use std::io;
use tui::widgets::ListState;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    InputProfileName,
    InputUserName,
    InputUserEmail,
    ListingProfiles, // Add this variant
}

pub fn handle_input(
    key: KeyEvent,
    input_mode: &mut InputMode,
    state: &mut ListState,
    options: &[&str],
    profile_name: &mut String,
    user_name: &mut String,
    user_email: &mut String,
) -> io::Result<()> {
    match input_mode {
        InputMode::Normal => {
            match key.code {
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
                    Some(0) => *input_mode = InputMode::InputProfileName,
                    Some(1) => switch_profile(),
                    Some(2) => update_profile(),
                    Some(3) => delete_profile(),
                    Some(4) => *input_mode = InputMode::ListingProfiles, // Switch to listing profiles
                    _ => {}
                },
                _ => {}
            }
        }
        InputMode::InputProfileName => match key.code {
            KeyCode::Char(c) => {
                profile_name.push(c);
            }
            KeyCode::Backspace => {
                profile_name.pop();
            }
            KeyCode::Enter => {
                if !profile_name.is_empty() {
                    *input_mode = InputMode::InputUserName;
                }
            }
            KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                profile_name.clear();
            }
            _ => {}
        },
        InputMode::InputUserName => match key.code {
            KeyCode::Char(c) => {
                user_name.push(c);
            }
            KeyCode::Backspace => {
                user_name.pop();
            }
            KeyCode::Enter => {
                if !user_name.is_empty() {
                    *input_mode = InputMode::InputUserEmail;
                }
            }
            KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                user_name.clear();
                profile_name.clear();
            }
            _ => {}
        },
        InputMode::InputUserEmail => match key.code {
            KeyCode::Char(c) => {
                user_email.push(c);
            }
            KeyCode::Backspace => {
                user_email.pop();
            }
            KeyCode::Enter => {
                if !user_email.is_empty() {
                    add_profile(profile_name, user_name, user_email)?;
                    *input_mode = InputMode::Normal;
                    profile_name.clear();
                    user_name.clear();
                    user_email.clear();
                }
            }
            KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                user_email.clear();
                user_name.clear();
                profile_name.clear();
            }
            _ => {}
        },
        InputMode::ListingProfiles => match key.code {
            KeyCode::Char('b') => {
                *input_mode = InputMode::Normal; // Go back to the main menu
            }
            _ => {}
        },
    }
    Ok(())
}

fn add_profile(profile_name: &str, user_name: &str, user_email: &str) -> io::Result<()> {
    let mut config = crate::config::load_config();
    config.profiles.insert(
        profile_name.to_string(),
        crate::config::GitConfig {
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
        },
    );
    crate::config::save_config(&config);
    Ok(())
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
