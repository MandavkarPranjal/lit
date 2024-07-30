use crossterm::event::{KeyCode, KeyEvent};
use std::io;
use tui::widgets::ListState;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    InputProfileName,
    InputUserName,
    InputUserEmail,
    ListingProfiles,
    DeleteProfile,
    ConfirmDeleteProfile,
    SwitchProfile,
}

pub fn handle_input(
    key: KeyEvent,
    input_mode: &mut InputMode,
    state: &mut ListState,
    delete_state: &mut ListState,
    switch_state: &mut ListState,
    options: &[&str],
    profile_name: &mut String,
    user_name: &mut String,
    user_email: &mut String,
    selected_profile_to_delete: &mut Option<String>,
    selected_profile_to_switch: &mut Option<String>,
    config: &mut crate::config::Config,
    delete_options: &[String],
    switch_options: &[String],
) -> io::Result<()> {
    match *input_mode {
        InputMode::Normal => match key.code {
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
            KeyCode::Enter => {
                match state.selected() {
                    Some(0) => *input_mode = InputMode::InputProfileName,
                    Some(1) => *input_mode = InputMode::SwitchProfile,
                    Some(2) => *input_mode = InputMode::InputUserName, // Assuming this is for updating profile
                    Some(3) => *input_mode = InputMode::DeleteProfile,
                    Some(4) => *input_mode = InputMode::ListingProfiles,
                    _ => {}
                }
            }
            _ => {}
        },
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
                    // Add profile to config
                    config.profiles.insert(
                        profile_name.clone(),
                        crate::config::GitConfig {
                            user_name: user_name.clone(),
                            user_email: user_email.clone(),
                        },
                    );
                    crate::config::save_config(config);
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
            KeyCode::Char('b') | KeyCode::Esc => {
                *input_mode = InputMode::Normal;
            }
            _ => {}
        },
        InputMode::DeleteProfile => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                let i = delete_state.selected().unwrap_or(0);
                if i > 0 {
                    delete_state.select(Some(i - 1));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let i = delete_state.selected().unwrap_or(0);
                if i < delete_options.len() - 1 {
                    delete_state.select(Some(i + 1));
                }
            }
            KeyCode::Enter => {
                if let Some(i) = delete_state.selected() {
                    *selected_profile_to_delete = Some(delete_options[i].clone());
                    *input_mode = InputMode::ConfirmDeleteProfile;
                }
            }
            KeyCode::Char('b') | KeyCode::Esc => {
                *input_mode = InputMode::Normal;
            }
            _ => {}
        },
        InputMode::SwitchProfile => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                let i = switch_state.selected().unwrap_or(0);
                if i > 0 {
                    switch_state.select(Some(i - 1));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let i = switch_state.selected().unwrap_or(0);
                if i < switch_options.len() - 1 {
                    switch_state.select(Some(i + 1));
                }
            }
            KeyCode::Enter => {
                if let Some(i) = switch_state.selected() {
                    let selected_profile = &switch_options[i];
                    if config.profiles.contains_key(selected_profile) {
                        // config.current_profile = selected_profile.clone();
                        let profile = config.profiles.get(selected_profile).unwrap();
                        crate::git_config::update_git_config(
                            &profile.user_name,
                            &profile.user_email,
                        );
                        *input_mode = InputMode::Normal;
                    }
                }
            }
            KeyCode::Char('b') | KeyCode::Esc => {
                *input_mode = InputMode::Normal;
            }
            _ => {}
        },
        InputMode::ConfirmDeleteProfile => match key.code {
            KeyCode::Char('y') => {
                if let Some(ref profile) = selected_profile_to_delete {
                    config.profiles.remove(profile);
                    crate::config::save_config(config);
                }
                *input_mode = InputMode::Normal;
                *selected_profile_to_delete = None;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                *selected_profile_to_delete = None;
            }
            _ => {}
        },
    }
    Ok(())
}
