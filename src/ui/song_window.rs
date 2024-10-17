use crate::Window;
use crate::{res_file, Message};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::LineHeight;
use iced::widget::{column, horizontal_space, image, row, stack, text};
use iced::{window, Element, Length, Task};
use iced::{Color, Size};

#[derive(Default)]
pub struct SongWindow {
    id: Option<window::Id>,
    size: Size,
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

            self.size = size;
        }

        ().into()
    }

    fn view(&self) -> Element<Message> {
        let dance_size = self.size.height / 8.0;
        let title_size = self.size.height / 20.0;
        let artist_size = self.size.height / 25.0;
        let next_dance_size = self.size.height / 20.0;

        // let dance_spacing = self.size.width / 7.0; // idk
        let song_spacing = self.size.width / 150.0;

        let cover_height = LineHeight::default().to_absolute(title_size.into())
            + song_spacing
            + LineHeight::default().to_absolute(artist_size.into());

        let element: Element<Message> = stack![
            column![
                text!("Gourmetta")
                    .size(dance_size)
                    .height(Length::Fill)
                    .align_y(Vertical::Bottom),
                row![
                    image(res_file!("icon.jpg")).height(cover_height),
                    column![
                        text!("Der DJ aus den Bergen").size(title_size),
                        text!("DJ Ötzi").size(artist_size),
                    ]
                    .spacing(song_spacing)
                ]
                .height(Length::Fill)
                .align_y(Vertical::Top)
                .spacing(song_spacing),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center),
            text!("Absturz")
                .size(next_dance_size)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Right)
                .align_y(Vertical::Bottom)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        element.explain(Color::new(0.0, 1.0, 0.0, 1.0))
    }
}
