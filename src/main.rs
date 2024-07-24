mod config;
mod git_config;

use clap::{Arg, Command};
use config::{load_config, save_config, GitConfig};

fn main() {
    let matches = Command::new("lit")
        .version("1.0")
        .author("Pranjal Mandavkar")
        .about("Git helper tool")
        .subcommand(
            Command::new("add-profile")
                .about("Add a new git configuration profile")
                .arg(
                    Arg::new("name")
                        .help("Name of the profile")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("user_name")
                        .help("Git user name")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("user_email")
                        .help("Git user email")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            Command::new("switch-profile")
                .about("Switch to a different git configuration profile")
                .arg(
                    Arg::new("name")
                        .help("Name of the profile to switch to")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("update-profile")
                .about("Update an existing git configuration profile")
                .arg(
                    Arg::new("name")
                        .help("Name of the profile to update")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("user_name")
                        .help("New git user name")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::new("user_email")
                        .help("New git user email")
                        .required(true)
                        .index(3),
                ),
        )
        .subcommand(
            Command::new("delete-profile")
                .about("Delete an existing git configuration profile")
                .arg(
                    Arg::new("name")
                        .help("Name of the profile to delete")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let mut config = load_config();

    if let Some(matches) = matches.subcommand_matches("add-profile") {
        let name = matches.get_one::<String>("name").unwrap();
        let user_name = matches.get_one::<String>("user_name").unwrap();
        let user_email = matches.get_one::<String>("user_email").unwrap();

        let git_config = GitConfig {
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
        };

        config.profiles.insert(name.to_string(), git_config);
        save_config(&config);
        println!("Profile '{}' added.", name);
    } else if let Some(matches) = matches.subcommand_matches("switch-profile") {
        let name = matches.get_one::<String>("name").unwrap();

        if config.profiles.contains_key(name) {
            config.current_profile = name.to_string();
            let profile = config.profiles.get(name).unwrap();
            git_config::update_git_config(&profile.user_name, &profile.user_email);
            save_config(&config);
            println!("Switched to profile '{}'.", name);
        } else {
            println!("Profile '{}' does not exist.", name);
        }
    } else if let Some(matches) = matches.subcommand_matches("update-profile") {
        let name = matches.get_one::<String>("name").unwrap();
        let user_name = matches.get_one::<String>("user_name").unwrap();
        let user_email = matches.get_one::<String>("user_email").unwrap();

        if let Some(profile) = config.profiles.get_mut(name) {
            profile.user_name = user_name.to_string();
            profile.user_email = user_email.to_string();
            save_config(&config);
            println!("Profile '{}' updated.", name);
        } else {
            println!("Profile '{}' does not exist.", name);
        }
    } else if let Some(matches) = matches.subcommand_matches("delete-profile") {
        let name = matches.get_one::<String>("name").unwrap();

        if config.profiles.remove(name).is_some() {
            if config.current_profile == *name {
                config.current_profile.clear();
            }
            save_config(&config);
            println!("Profile '{}' deleted.", name);
        } else {
            println!("Profile '{}' does not exist.", name);
        }
    }
}
