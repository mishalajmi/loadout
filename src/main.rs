use crate::pages::setup::{SetupPage, SetupPageEvent};
use iced::window::{Position, Settings};
use iced::{window, Element, Size, Task, Theme};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

mod pages {
    pub mod setup;
}

#[derive(Debug, Clone)]
enum Message {
    UpdateWidth(String),
    UpdateHeight(String),
    PageTransition(String),
    SetupPageTransition(SetupPageEvent)
}

#[derive(Debug, Clone)]
enum LoadoutPages {
    LoadoutSetupPage(SetupPage)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Games {
    game_title: String,
    game_cover_art: String,
    game_dir_location: PathBuf,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum GameLauncher {
    Steam,
    Epic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Platform {
    launcher: GameLauncher,
    game_directory: PathBuf,
    icon: String,
    games: Vec<Games>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct LoadoutDisplay {
    width: f32,
    height: f32,
    centered: bool,
}

impl Default for LoadoutDisplay {
    fn default() -> Self {
        LoadoutDisplay {
            height: 768.,
            width: 1160.,
            centered: true,
        }
    }
}

impl LoadoutDisplay {
    fn update_screen_width(&mut self, value: &str) {
        self.width = value.parse::<f32>().unwrap_or(self.width)
    }

    fn update_screen_height(&mut self, value: &str) {
        self.height = value.parse::<f32>().unwrap_or(self.height)
    }

    fn get_screen_width(self) -> f32 {
        self.width
    }

    fn get_screen_height(self) -> f32 {
        self.height
    }

    fn get_display_position(self) -> Position {
        if self.centered {
            Position::Centered
        } else {
            Position::Default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoadoutOptions {
    display: LoadoutDisplay,
    theme: String,
    run_on_startup: bool,
}

impl Default for LoadoutOptions {
    fn default() -> Self {
        LoadoutOptions {
            display: LoadoutDisplay::default(),
            theme: "CatppuccinMocha".into(),
            run_on_startup: false,
        }
    }
}

impl LoadoutOptions {
    fn load_or_default() -> Self {
        Self::load_from_file().unwrap_or_else(|_| Self::default_options())
    }

    fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        let file = std::fs::read_to_string(config_path)?;
        let options = serde_json::from_str(&file)?;
        Ok(options)
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Could not find config directory")?;
        path.push("Loadout");
        fs::create_dir_all(&path)?;
        path.push(".loadout");
        Ok(path)
    }

    fn default_options() -> Self {
        Self {
            display: LoadoutDisplay::default(),
            theme: "CatppuccinMocha".to_string(),
            run_on_startup: false,
        }
    }

    fn get_theme(self) -> Theme {
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
    title: String,
    options: LoadoutOptions,
    platforms: Vec<Platform>,
    // TODO: create a vec of pages for the whole application
    pages: Vec<SetupPage>,
}

impl Default for Loadout {
    // setup platforms here
    // call something like platforms.load_from_config(&options.config) for e.g.
    fn default() -> Self {
        Self {
            title: String::from("Loadout"),
            options: LoadoutOptions::load_or_default(),
            platforms: Vec::new(),
            pages: vec![SetupPage::default()],
        }
    }
}

impl Loadout {
    fn new(self) -> (Self, Task<Message>) {
        (Loadout::default(), Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateWidth(w) => self.options.display.update_screen_width(&w[..]),
            Message::UpdateHeight(h) => self.options.display.update_screen_height(&h[..]),
            Message::PageTransition(p) => println!("Transitioning to new page"),
            Message::SetupPageTransition(event) => self.pages.get(0).unwrap().clone().select_directory("hello".to_string()),
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let current_page = self.pages.get(0);

        current_page.unwrap().view()
    }
}

fn main() -> iced::Result {
    let initial_options = RefCell::new(LoadoutOptions::load_or_default());
    iced::application(Loadout::default, Loadout::update, Loadout::view)
        .theme(initial_options.borrow().clone().get_theme())
        .window(Settings {
            size: Size {
                width: initial_options.borrow().display.get_screen_width(),
                height: initial_options.borrow().display.get_screen_height(),
            },
            position: initial_options.borrow().display.get_display_position(),
            ..window::Settings::default()
        })
        .run()
}
