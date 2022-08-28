use directories::ProjectDirs;
use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &str = "shpalery.toml";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub amount: i32,
    pub fetch_type: String, //  one of hot, top(hour, day, week, month, year, all)
    pub subreddits: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            amount: 10,
            fetch_type: "hot".to_string(),
            subreddits: vec!["wallpaper".to_string(), "wallpapers".to_string()],
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let project_dir =
            ProjectDirs::from("com", "elkrammer", "shpalery").expect("Couldn't create project dir");
        let config_dir = project_dir.config_dir();
        let config_file = project_dir.config_dir().join(CONFIG_FILE);

        if !config_dir.exists() || !config_file.exists() {
            return Config::default();
        }

        let toml_text = fs::read_to_string(config_file).expect("Problem reading config file");
        let cfg: Config = toml::from_str(&toml_text).expect("Problem converting TOML to string");
        return cfg;
    }
}
