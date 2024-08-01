use crossterm::event::{KeyCode, KeyEvent};
use std::io;
use tui::widgets::ListState;

use crate::config::{save_config, Config};

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    InputProfileName,
    InputUserName,
    InputUserEmail,
    UpdateProfileUserName,
    UpdateProfileUserEmail,
    ListingProfiles,
    DeleteProfile,
    SwitchProfile,
    UpdateProfile,
    ConfirmDeleteProfile,
}

pub fn handle_input(
    key: KeyEvent,
    input_mode: &mut InputMode,
    profile_name: &mut String,
    user_name: &mut String,
    user_email: &mut String,
    state: &mut ListState,
    options: &[&str],
    selected_profile_to_delete: &mut Option<String>,
    delete_state: &mut ListState,
    delete_options: &[String],
    _selected_profile_to_switch: &mut Option<String>,
    switch_state: &mut ListState,
    switch_options: &[String],
    selected_profile_to_update: &mut Option<String>,
    update_state: &mut ListState,
    update_options: &[String],
    config: &mut Config,
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
            KeyCode::Enter => match state.selected() {
                Some(0) => *input_mode = InputMode::InputProfileName,
                Some(1) => *input_mode = InputMode::SwitchProfile,
                Some(2) => *input_mode = InputMode::UpdateProfile,
                Some(3) => *input_mode = InputMode::DeleteProfile,
                Some(4) => *input_mode = InputMode::ListingProfiles,
                _ => {}
            },
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
                    save_config(config);
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
        InputMode::UpdateProfileUserName => match key.code {
            KeyCode::Char(c) => {
                user_name.push(c);
            }
            KeyCode::Backspace => {
                user_name.pop();
            }
            KeyCode::Enter => {
                if !user_name.is_empty() {
                    *input_mode = InputMode::UpdateProfileUserEmail;
                }
            }
            KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                user_name.clear();
            }
            _ => {}
        },
        InputMode::UpdateProfileUserEmail => match key.code {
            KeyCode::Char(c) => {
                user_email.push(c);
            }
            KeyCode::Backspace => {
                user_email.pop();
            }
            KeyCode::Enter => {
                if !user_email.is_empty() {
                    if let Some(profile) = selected_profile_to_update.clone() {
                        if let Some(profile_data) = config.profiles.get_mut(&profile) {
                            profile_data.user_name = user_name.clone();
                            profile_data.user_email = user_email.clone();
                            save_config(config);
                        }
                    }
                    selected_profile_to_update.take();
                    user_name.clear();
                    user_email.clear();
                    *input_mode = InputMode::Normal;
                }
            }
            KeyCode::Esc => {
                *input_mode = InputMode::Normal;
                user_email.clear();
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
        InputMode::UpdateProfile => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                let i = update_state.selected().unwrap_or(0);
                if i > 0 {
                    update_state.select(Some(i - 1));
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let i = update_state.selected().unwrap_or(0);
                if i < update_options.len() - 1 {
                    update_state.select(Some(i + 1));
                }
            }
            KeyCode::Enter => {
                if let Some(i) = update_state.selected() {
                    *selected_profile_to_update = Some(update_options[i].clone());
                    *input_mode = InputMode::UpdateProfileUserName;
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
                    save_config(config);
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
