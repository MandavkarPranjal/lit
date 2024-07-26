use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub profiles: HashMap<String, GitConfig>,
    pub current_profile: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitConfig {
    pub user_name: String,
    pub user_email: String,
}

pub fn load_config() -> Config {
    let config_str = fs::read_to_string("config.json").unwrap_or_else(|_| String::from("{}"));
    serde_json::from_str(&config_str).unwrap_or_else(|_| Config {
        profiles: HashMap::new(),
        current_profile: String::new(),
    })
}

pub fn save_config(config: &Config) {
    let config_str = serde_json::to_string_pretty(config).unwrap();
    fs::write("config.json", config_str).unwrap();
}

pub fn list_profiles() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();
    println!("Profiles:");
    for (name, profile) in config.profiles {
        println!("Profile: {}", name);
        println!("  User Name: {}", profile.user_name);
        println!("  User Email: {}", profile.user_email);
    }
    Ok(())
}
