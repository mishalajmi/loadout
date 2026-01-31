use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Color, Element, Length};
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct SetupPage {
    pub directory_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum SetupPageEvent {
    DirectorySelected(String),
}

impl SetupPage {
    pub fn new() -> Self {
        Self {
            directory_path: PathBuf::default(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let display = row![column![
            text("Setup your first game library")
                .size(24)
                .width(Length::Fill)
                .height(Length::Fill)
                .color(Color::from_rgba8(255, 255, 255, 1.0))
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            button("Select a directory")
                .width(Length::Fixed(128.))
                .height(Length::Fixed(64.))
                .on_press(SetupPageEvent::DirectorySelected("test".to_string()).into())
        ]];
        container(display).padding(10).into()
    }

    pub fn select_directory(&mut self, dir: String) {
        let path = PathBuf::from(dir);
        if !path.exists() || !path.is_dir() {
            println!(
                "Provided shit is neither a directory nor does it exist, {:?}",
                path
            );
            return;
        }

        self.directory_path = path.canonicalize().unwrap();
        println!("Using this path for directory: {:?}", self.directory_path);
    }
}

impl From<SetupPageEvent> for Message {
    fn from(value: SetupPageEvent) -> Self {
        Self::SetupPageTransition(value)
    }
}
