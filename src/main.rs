mod config;
mod error;
mod pages;
mod game;

use crate::config::{Config, GameDirectory, Launcher};
use error::LoadoutError;
use iced::{window, Element, Task};
use std::path::PathBuf;
use crate::game::Game;
use crate::pages::{LibraryPage, SetupPage};

#[derive(Debug, Clone)]
enum Page {
    Library(LibraryPage),
    AddDirectory(SetupPage),
    GameDetails(String),
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
    LaunchGame(Game),
}

#[derive(Debug, Clone)]
struct Loadout {
    config: Config,
    current_page: Page,
}

impl Default for Loadout {
    fn default() -> Self {
        let config = Config::load();
        let current_page = if config.directories.is_empty() {
            Page::AddDirectory(SetupPage::default())
        } else {
            let games: Vec<Game> = config
                .directories
                .iter()
                .flat_map(|dir| dir.games.clone())
                .collect();
            Page::Library(LibraryPage::new(games))
        };

        Self {
            config,
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
                                    let games = self
                                        .config
                                        .directories
                                        .iter()
                                        .flat_map(|dir| dir.games.clone())
                                        .collect();

                                    self.current_page = Page::Library(LibraryPage::new(games));
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
                                let games = self
                                    .config
                                    .directories
                                    .iter()
                                    .flat_map(|dir| dir.games.clone())
                                    .collect();
                                self.current_page = Page::Library(LibraryPage::new(games));
                            }
                        }
                    }
                }
            }
            Message::LaunchGame(game) => {
                if let Err(e) = game.load() {
                    eprintln!("Failed to launch game: {}", e);
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::text;

        match &self.current_page {
            Page::Library(library) => library.view(),
            Page::Settings => text("Settings page").into(),
            Page::AddDirectory(setup) => setup.view(),
            Page::GameDetails(_games) => text("Game details").into(),
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
