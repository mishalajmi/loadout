use crate::{game, Message, Page};
use iced::widget::{Column, button, column, container, image, row, scrollable, text};
use iced::{Border, Color, Element, Length, Shadow, Theme};
use iced::alignment::Horizontal;
use crate::game::Game;

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
                .map(|game| game.build_display_card())
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
