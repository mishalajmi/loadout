use crate::Message;
use crate::error::LoadoutError;
use iced::alignment::Horizontal;
use iced::widget::{Column, button, container, image, text};
use iced::{Border, Color, Element, Length, Shadow, Theme};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub cover_art: PathBuf,
    pub directory: PathBuf,
}

impl Game {
    pub fn new(title: &str, cover_art: PathBuf, directory: PathBuf) -> Self {
        // do some validations here
        //
        let title = title.to_string();
        Self {
            title,
            cover_art,
            directory,
        }
    }

    pub fn load(&self) -> Result<(), LoadoutError> {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", self.directory.as_os_str().to_str().unwrap()])
                .spawn()
                .map_err(|e| LoadoutError::GameLaunchError {
                    path: self.directory.clone(),
                    source: e,
                })?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new("xdg-open")
                .arg(self.directory.as_os_str())
                .spawn()
                .map_err(|e| LoadoutError::GameLaunchError {
                    path: self.directory.clone(),
                    source: e,
                })?;
        }
        Ok(())
    }

    pub fn build_display_card(&self) -> Column<'_, Message> {
        // Card background colors
        let card_bg = Color::from_rgb(0.15, 0.15, 0.18);
        let hover_bg = Color::from_rgb(0.18, 0.18, 0.22);

        // Artwork with rounded corners and shadow
        let artwork = if self.cover_art.as_os_str().is_empty() {
            container(text("N/A").size(48).style(|_theme: &Theme| text::Style {
                color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
            }))
            .width(200)
            .height(280)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.12, 0.12, 0.15))),
                border: Border {
                    radius: 12.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        } else {
            container(image(self.cover_art.as_os_str()).width(200).height(280))
                .width(200)
                .height(280)
                .style(|_theme: &Theme| container::Style {
                    border: Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
        };

        // Game title with truncation
        let title = text(self.title.as_str())
            .size(18)
            .width(200)
            .style(|_theme: &Theme| text::Style {
                color: Some(Color::from_rgb(0.95, 0.95, 0.95)),
            });

        // Launch button with gradient-like styling
        let launch_button = button(
            text("Launch Game")
                .size(14)
                .align_x(Horizontal::Center)
                .style(|_theme: &Theme| text::Style {
                    color: Some(Color::WHITE),
                }),
        )
        .width(200)
        .padding([10, 16])
        .style(move |theme: &Theme, status| {
            let palette = theme.extended_palette();
            match status {
                button::Status::Active => button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.4, 0.5, 0.9))),
                    text_color: Color::WHITE,
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 8.0,
                    },
                    snap: false,
                },
                button::Status::Hovered => button::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.5, 0.6, 1.0))),
                    text_color: Color::WHITE,
                    border: Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                        offset: iced::Vector::new(0.0, 4.0),
                        blur_radius: 12.0,
                    },
                    snap: false,
                },
                _ => button::primary(theme, status),
            }
        })
        .on_press(Message::LaunchGame(self.clone()));

        // Card container with background and shadow
        let card_content = iced::widget::column![artwork, title, launch_button]
            .spacing(12)
            .align_x(Horizontal::Center)
            .padding(16);

        // Wrap in a styled container for the card background
        iced::widget::column![container(card_content).style(move |_theme: &Theme| {
            container::Style {
                text_color: None,
                background: Some(iced::Background::Color(card_bg)),
                border: Border {
                    radius: 16.0.into(),
                    width: 1.0,
                    color: Color::from_rgb(0.25, 0.25, 0.28),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                snap: false,
            }
        })]
    }
}
