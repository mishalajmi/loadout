use crate::config::Game;
use crate::{Message, Page};
use iced::widget::{Column, button, column, container, image, row, scrollable, text};
use iced::{Border, Color, Element, Length, Shadow, Theme};
use iced::alignment::Horizontal;

#[derive(Debug, Clone, Default)]
pub struct LibraryPage {
    pub games: Vec<Game>,
}

impl LibraryPage {
    pub fn new(games: Vec<Game>) -> Self {
        Self { games }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.games.is_empty() {
            return self.empty_state();
        }
        let mut card_rows: Vec<Element<'_, Message>> = Vec::new();
        for chunk in self.games.chunks(4) {
            let cards: Vec<Element<'_, Message>> = chunk
                .iter()
                .map(|game| build_game_card(game).into())
                .collect();
            let card_row = row(cards).spacing(20).into();
            card_rows.push(card_row);
        }
        let grid = column(card_rows).spacing(20).padding(20);

        scrollable(container(grid).width(Length::Fill).height(Length::Fill)).into()
    }

    fn empty_state(&self) -> Element<'_, Message> {
        container(
            column![
                text("No games found")
                    .size(32)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
                    }),
                text("Add a game directory to get started")
                    .size(16)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(Color::from_rgb(0.6, 0.6, 0.6)),
                    }),
                button(
                    text("Add Directory")
                        .size(16)
                        .align_x(Horizontal::Center)
                )
                .padding([12, 24])
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    button::Style {
                        background: Some(iced::Background::Color(palette.primary.strong.color)),
                        text_color: palette.primary.strong.text,
                        border: Border::default().rounded(8),
                        ..button::primary(theme, status)
                    }
                })
                .on_press(Message::NavigateTo(Page::AddDirectory(
                    crate::pages::setup::SetupPage::default()
                )))
            ]
                .spacing(20)
                .align_x(Horizontal::Center),
        )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}

fn build_game_card(game: &Game) -> Column<'_, Message> {
    // Card background colors
    let card_bg = Color::from_rgb(0.15, 0.15, 0.18);
    let hover_bg = Color::from_rgb(0.18, 0.18, 0.22);

    // Artwork with rounded corners and shadow
    let artwork = if game.cover_art.is_empty() {
        container(
            text("N/A")
                .size(48)
                .style(|_theme: &Theme| text::Style {
                    color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
                }),
        )
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
        container(image(&game.cover_art).width(200).height(280))
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
    let title = text(&game.title)
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
        .on_press(Message::LaunchGame(game.directory.clone()));

    // Card container with background and shadow
    let card_content = column![artwork, title, launch_button]
        .spacing(12)
        .align_x(Horizontal::Center)
        .padding(16);

    // Wrap in a styled container for the card background
    column![container(card_content)
        .style(move |_theme: &Theme| container::Style {
            text_color: None,background: Some(iced::Background::Color(card_bg)),
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
        snap: false,})]
}
