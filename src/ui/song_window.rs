use crate::res_file;
use crate::{Message, Window};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::{column, image, row, stack, text};
use iced::Size;
use iced::{window, Element, Length, Task, Theme};

pub struct SongWindow {
    id: Option<window::Id>,
    size: Size,
}

impl Default for SongWindow {
    fn default() -> Self {
        Self {
            id: None,
            size: Size::new(1.0, 1.0),
        }
    }
}

impl Window for SongWindow {
    fn set_id(&mut self, id: window::Id) {
        self.id = Some(id);
    }

    fn title(&self) -> String {
        "Song Window".to_owned()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let Some(id) = self.id else {
            return ().into();
        };

        if let Message::WindowResized((ev_id, size)) = message {
            if ev_id != id {
                return ().into();
            };

            self.size = size.max(Size::new(1.0, 1.0));
        }

        ().into()
    }

    fn view(&self) -> Element<Message> {
        let dance_size = self.size.height / 8.0;
        let title_size = self.size.height / 20.0;
        let artist_size = self.size.height / 25.0;
        let next_dance_size = self.size.height / 20.0;

        let dance_spacing = self.size.width / 7.0;
        let song_spacing = self.size.width / 150.0;

        let cover_height = LineHeight::default().to_absolute(title_size.into())
            + song_spacing
            + LineHeight::default().to_absolute(artist_size.into());

        let text_dance = text!("Gourmetta")
            .size(dance_size)
            .height(Length::Fill)
            .align_y(Vertical::Bottom);

        let column_title_artist = column![
            text!("Der DJ aus den Bergen").size(title_size),
            text!("DJ Ötzi").size(artist_size),
        ]
        .spacing(song_spacing);

        let row_bottom = (if true {
            row![
                image(res_file!("icon.jpg")).height(cover_height),
                column_title_artist
            ]
        } else {
            row![column_title_artist]
        })
        .height(Length::Fill)
        .align_y(Vertical::Top)
        .spacing(song_spacing);

        let column_center = column![text_dance, row_bottom]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .spacing(dance_spacing);

        (if true {
            stack![
                column_center,
                text!("Absturz")
                    .size(next_dance_size)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Right)
                    .align_y(Vertical::Bottom)
            ]
        } else {
            stack![column_center]
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
