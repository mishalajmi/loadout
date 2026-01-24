use iced::widget::{column, container, row, text};
use iced::window::{Position, Settings};
use iced::{window, Color, Element, Length, Size, Task, Theme};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum Message {
    UpdateWidth(String),
    UpdateHeight(String),
}
#[derive(Debug, Clone, Copy)]
enum GameLauncher {
    Steam,
    Epic,
    Custom,
}

#[derive(Debug, Clone)]
struct Platform {
    launcher: GameLauncher,
    game_directory: PathBuf,
    icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            _ => Theme::CatppuccinMocha
        }
    }
}

#[derive(Debug, Clone)]
struct Loadout {
    title: String,
    options: LoadoutOptions,
    platforms: Vec<Platform>,
}

impl Default for Loadout {
    fn default() -> Self {
        Self {
            title: String::from("Loadout"),
            options: LoadoutOptions::load_or_default(),
            platforms: Vec::new(),
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
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let display = row![
            column![
                text("test")
                    .size(24)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .color(Color::from_rgba8(255, 100, 50, 1.0))
            ]
            .spacing(18),
            column![
                text("test")
                    .size(24)
                    .width(Length::FillPortion(3))
                    .height(Length::FillPortion(3)),
            ]
        ];
        container(display).padding(10).into()
    }
}

fn main() -> iced::Result {
    let initial_options = LoadoutOptions::load_or_default();
    iced::application(Loadout::default, Loadout::update, Loadout::view)
        .theme(initial_options.clone().get_theme())
        .window(Settings {
            size: Size {
                width: initial_options.clone().display.get_screen_width(),
                height: initial_options.clone().display.get_screen_height(),
            },
            position: initial_options.clone().display.get_display_position(),
            ..window::Settings::default()
        })
        .run()
}
