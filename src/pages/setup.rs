use crate::{Message, Action};
use iced::alignment::{Horizontal};
use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length};
use rfd::FileDialog;
use std::path::PathBuf;
use crate::error::{LoadoutError, Result};

#[derive(Debug, Clone, Default)]
pub struct SetupPage {
    pub directory_path: PathBuf,
}

impl SetupPage {
    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Add a game directory")
                .size(24)
                .color(Color::WHITE),
            button("Select Directory")
                .padding(10)
                .on_press(Message::DirectoryAction(Action::SelectDirectory)),
            button("Cancel")
                .padding(10)
                .on_press(Message::DirectoryAction(Action::Cancel)),
        ]
            .spacing(20)
            .align_x(Horizontal::Center);

        container(content)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    pub fn select_directory(&mut self) -> Result<()>{
        let path = FileDialog::new()
            .pick_folder()
            .ok_or(LoadoutError::NoDirectorySelected)?;
        self.directory_path = path;
        println!("Using this path for directory: {:?}", self.directory_path);
        Ok(())
    }
}


