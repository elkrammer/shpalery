use colored::Colorize;
use directories::ProjectDirs;
use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE: &str = "shpalery.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub amount: i32,
    pub fetch_type: String, // one of hot, top(hour, day, week, month, year, all)
    pub subreddits: Vec<String>, // "wallpaper", "wallpapers", "EarthPorn", "SkyPorn"
    pub download_dir: PathBuf, // Linux: $HOME/.local/share/, Windows: {FOLDERID_RoamingAppData}, Mac: $HOME/Library/Application Support
}

impl Default for Config {
    fn default() -> Self {
        Self {
            amount: 10,
            fetch_type: "hot".to_string(),
            subreddits: vec!["wallpaper".to_string(), "wallpapers".to_string()],
            download_dir: Self::get_wallpapers_data_dir(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_file = Self::project_dir().config_dir().join(CONFIG_FILE);
        Figment::from(Serialized::defaults(Self::default()))
            .merge(Toml::file(config_file))
            .merge(Env::prefixed("APP_"))
            .extract()
            .unwrap()
    }

    pub fn project_dir() -> ProjectDirs {
        ProjectDirs::from("com", "elkrammer", "shpalery").expect("Couldn't set project dir")
    }

    pub fn get_data_dir() -> PathBuf {
        let data_dir = Self::project_dir().data_dir().to_owned();
        if !data_dir.exists() {
            println!("Data dir {} doesn't exist, creating...", data_dir.display());
            Self::create_data_dir(&data_dir).unwrap();
        }
        data_dir.to_path_buf()
    }

    pub fn get_database_file() -> PathBuf {
        Self::project_dir().data_dir().join("shpalery.db")
    }

    pub fn get_wallpapers_data_dir() -> PathBuf {
        let wallpapers_dir = Self::get_data_dir().join("wallpapers");
        if !wallpapers_dir.exists() {
            Self::create_data_dir(&wallpapers_dir).unwrap();
        }
        wallpapers_dir
    }

    pub fn create_data_dir(data_dir: &Path) -> Result<(), std::io::Error> {
        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }
        Ok(())
    }

    pub fn print_config(&self) {
        println!(
            "\
            {}{}\n\
            {}{}\n\
            {}{}\n\
            ",
            format!("Wallpapers Path : ").blue().bold(),
            Self::get_wallpapers_data_dir().display(),
            format!("Database Path   : ").yellow().bold(),
            Self::get_database_file().display(),
            format!("Download Total  : ").green(),
            self.amount
        );
    }
}
