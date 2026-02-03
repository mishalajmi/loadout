mod error;
mod pages {
    pub mod setup;
}

use crate::pages::setup::SetupPage;
use error::{LoadoutError, Result};
use iced::window::Position;
use iced::{window, Element, Size, Task, Theme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Page {
    Library,
    AddDirectory(SetupPage),
    Settings,
}

#[derive(Debug, Clone)]
pub enum Action {
    SelectDirectory,
    Cancel,
}

#[derive(Debug, Clone)]
enum Message {
    NavigateTo(Page),
    DirectoryAction(Action),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Game {
    title: String,
    cover_art: String,
    directory: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Launcher {
    Steam,
    Epic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GameDirectory {
    launcher: Launcher,
    path: PathBuf,
    icon: String,
    games: Vec<Game>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct DisplaySettings {
    width: f32,
    height: f32,
    centered: bool,
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
    fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    fn position(&self) -> Position {
        if self.centered {
            Position::Centered
        } else {
            Position::Default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    display: DisplaySettings,
    theme: String,
    run_on_startup: bool,
    directories: Vec<GameDirectory>,
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
    fn load() -> Self {
        Self::load_from_file().unwrap_or_default()
    }

    fn load_from_file() -> Result<Self> {
        let path = Self::config_path()?;
        let content =
            std::fs::read_to_string(&path).map_err(|e| LoadoutError::ConfigReadError {
                path: path.clone(),
                source: e,
            })?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).map_err(|e| LoadoutError::ConfigWriteError {
            path: path.clone(),
            source: e,
        })?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().ok_or(LoadoutError::ConfigDirNotFound)?;
        path.push("Loadout");
        fs::create_dir_all(&path).map_err(|e| LoadoutError::ConfigDirCreateError {
            path: path.clone(),
            source: e,
        })?;
        path.push("config.json");
        Ok(path)
    }
    fn theme(&self) -> Theme {
        match self.theme.as_str() {
            "CatppuccinMocha" => Theme::CatppuccinMocha,
            "Light" => Theme::CatppuccinFrappe,
            "Dark" => Theme::TokyoNight,
            _ => Theme::CatppuccinMocha,
        }
    }
}

#[derive(Debug, Clone)]
struct Loadout {
    config: Config,
    current_page: Page,
}

impl Default for Loadout {
    fn default() -> Self {
        let options = Config::load();
        let current_page = if options.directories.is_empty() {
            Page::AddDirectory(SetupPage::default())
        } else {
            Page::Library
        };

        Self {
            config: Config::load(),
            current_page,
        }
    }
}

impl Loadout {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(page) => self.current_page = page,
            Message::DirectoryAction(action) => {
                if let Page::AddDirectory(ref mut page) = self.current_page {
                    match action {
                        Action::SelectDirectory => match page.select_directory() {
                            Ok(_) => {
                                if !page.directory_path.as_os_str().is_empty() {
                                    let new_dir = GameDirectory {
                                        launcher: Launcher::Custom,
                                        path: page.directory_path.clone(),
                                        icon: String::new(),
                                        games: Vec::new(),
                                    };
                                    self.config.directories.push(new_dir);
                                    if let Err(e) = self.config.save() {
                                        eprintln!("Failed to save config: {}", e);
                                    }
                                    self.current_page = Page::Library
                                }
                            }
                            Err(LoadoutError::NoDirectorySelected) => {
                                eprintln!("You didn't select a directory")
                            }
                            Err(e) => {
                                eprintln!("Error selecting directory: {}", e);
                            }
                        },
                        Action::Cancel => {
                            if !self.config.directories.is_empty() {
                                self.current_page = Page::Library;
                            }
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::{button, column, text};

        match &self.current_page {
            Page::Library => column![
                text("Your Game Library").size(24),
                text(format!(
                    "{} directories configured",
                    self.config.directories.len()
                )),
                button("Add Directory").on_press(Message::NavigateTo(Page::AddDirectory(
                    SetupPage::default()
                ))),
            ]
            .padding(20)
            .spacing(10)
            .into(),
            Page::Settings => text("Settings page").into(),
            Page::AddDirectory(page) => page.view(),
        }
    }
}

fn main() -> iced::Result {
    let config = Config::load();
    iced::application(Loadout::default, Loadout::update, Loadout::view)
        .theme(config.theme())
        .window(window::Settings {
            size: config.display.size(),
            position: config.display.position(),
            ..window::Settings::default()
        })
        .run()
}
