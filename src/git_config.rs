use std::process::Command;

pub fn update_git_config(user_name: &str, user_email: &str) {
    Command::new("git")
        .args(&["config", "--global", "user.name", user_name])
        .output()
        .expect("Failed to update git user.name");

    Command::new("git")
        .args(&["config", "--global", "user.email", user_email])
        .output()
        .expect("Failed to update git user.email");
}
