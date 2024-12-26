use crate::{DanceInterpreter, Message};
use crate::Window;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::{column, horizontal_space, image, row, stack, Text};
use iced::Size;
use iced::{window, Element, Length};

pub struct SongWindow {
    pub id: Option<window::Id>,
    pub size: Size,
}

impl Window for SongWindow {
    fn on_create(&mut self, id: window::Id) {
        self.id = Some(id);
    }

    fn on_resize(&mut self, size: Size) {
        self.size = size;
    }
}

impl Default for SongWindow {
    fn default() -> Self {
        Self {
            id: None,
            size: Size::new(1.0, 1.0),
        }
    }
}

impl SongWindow {
    pub fn view(&self, state: &DanceInterpreter) -> Element<Message> {
        let Some(song_info) = state.song_info.as_ref() else {
            return horizontal_space().into();
        };

        let dance_size = self.size.height / 8.0;
        let title_size = self.size.height / 20.0;
        let artist_size = self.size.height / 25.0;
        let next_dance_size = self.size.height / 20.0;

        let dance_spacing = self.size.height / 35.0;
        let song_spacing = self.size.height / 150.0;

        let cover_height = LineHeight::default().to_absolute(title_size.into())
            + song_spacing
            + LineHeight::default().to_absolute(artist_size.into());

        let text_dance = Text::new(song_info.dance.to_owned())
            .size(dance_size)
            .height(Length::Fill)
            .align_y(Vertical::Bottom);

        let column_title_artist = column![
            Text::new(song_info.title.to_owned()).size(title_size),
            Text::new(song_info.artist.to_owned()).size(artist_size),
        ]
        .spacing(song_spacing);

        let row_bottom = (if let Some(image_handle) = song_info.album_art.as_ref() {
            row![
                image(image_handle).height(cover_height),
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

        (if let Some(next_song_info) = state.next_song_info.as_ref() {
            stack![
                column_center,
                Text::new(next_song_info.dance.to_owned())
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
}
