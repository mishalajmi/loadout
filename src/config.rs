use std::fs;
use std::path::PathBuf;
use iced::{window, Size, Theme};
use serde::{Deserialize, Serialize};
use crate::error::LoadoutError;
use crate::game::Game;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Launcher {
    Steam,
    Epic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDirectory {
    pub launcher: Launcher,
    pub path: PathBuf,
    pub icon: String,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub width: f32,
    pub height: f32,
    pub centered: bool,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        DisplaySettings {
            width: 1160.,
            height: 768.,
            centered: true,
        }
    }
}

impl DisplaySettings {
    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn position(&self) -> window::Position {
        if self.centered {
            window::Position::Centered
        } else {
            window::Position::Default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplaySettings,
    pub theme: String,
    pub run_on_startup: bool,
    pub directories: Vec<GameDirectory>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            display: DisplaySettings::default(),
            theme: "CatppuccinMocha".into(),
            run_on_startup: false,
            directories: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::load_from_file().unwrap_or_default()
    }

    pub fn load_from_file() -> crate::error::Result<Self> {
        let path = Self::config_path()?;
        let content =
            fs::read_to_string(&path).map_err(|e| LoadoutError::ConfigReadError {
                path: path.clone(),
                source: e,
            })?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> crate::error::Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).map_err(|e| LoadoutError::ConfigWriteError {
            path: path.clone(),
            source: e,
        })?;
        Ok(())
    }

    pub fn config_path() -> crate::error::Result<PathBuf> {
        let mut path = dirs::config_dir().ok_or(LoadoutError::ConfigDirNotFound)?;
        path.push("Loadout");
        fs::create_dir_all(&path).map_err(|e| LoadoutError::ConfigDirCreateError {
            path: path.clone(),
            source: e,
        })?;
        path.push("config.json");
        Ok(path)
    }
    pub fn theme(&self) -> Theme {
        match self.theme.as_str() {
            "CatppuccinMocha" => Theme::CatppuccinMocha,
            "Light" => Theme::CatppuccinFrappe,
            "Dark" => Theme::TokyoNight,
            _ => Theme::CatppuccinMocha,
        }
    }
}