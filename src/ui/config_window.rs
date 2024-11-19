use std::path::PathBuf;
use crate::{Message, Window};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, checkbox, text};
use iced::{window, Element, Length, Size, Task, Theme};
use iced_aw::menu::Item;
use iced_aw::{menu_bar, menu_items, Menu};
use rfd::FileDialog;
use crate::dataloading::m3uloader::load_tag_data_from_m3u;

#[derive(Default)]
pub struct ConfigWindow {
    id: Option<window::Id>,
    size: Size,
}

impl Window for ConfigWindow {
    fn set_id(&mut self, id: window::Id) {
        self.id = Some(id);
    }

    fn title(&self) -> String {
        "Config Window".to_owned()
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

        if let Message::OpenPlaylist = message {
            // Open playlist file
            let file = FileDialog::new()
                .add_filter("Playlist", &["m3u", "m3u8"])
                .add_filter("Any(*)", &["*"])
                .set_title("Select playlist file")
                .set_directory(dirs::audio_dir().unwrap_or(dirs::home_dir().unwrap_or(PathBuf::from("."))))
                .pick_file();

            if file.is_some()  {
                println!("Selected file: {:?}", file);
                let playlist = load_tag_data_from_m3u(&file.unwrap());
            }
        }

        ().into()
    }

    fn view(&self) -> Element<Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(150.0).offset(15.0).spacing(5.0);
        let mb = menu_bar!(
            (button(
                text("File").align_y(Vertical::Center)
            ).padding([4, 8]).style(button::secondary),
            menu_tpl_1(
                menu_items!(
                    (button(
                        text("Open Playlist File").align_y(Vertical::Center).align_x(Horizontal::Left)
                    ).padding([4, 8]).on_press(Message::OpenPlaylist)
                    .width(Length::Fill)
                    .style(button::secondary))
                (button(
                        text("Exit").align_y(Vertical::Center).align_x(Horizontal::Left)
                    ).padding([4, 8])
                    .width(Length::Fill)
                    .style(button::secondary))
            )).spacing(5.0))
            (button(
                text("Edit").align_y(Vertical::Center)
            ).padding([4, 8]).style(button::secondary),
            menu_tpl_1(
                menu_items!(
                    (button(
                        text("Import Playlistview").align_y(Vertical::Center).align_x(Horizontal::Left)
                    ).padding([4, 8])
                    .width(Length::Fill)
                    .style(button::secondary))
                (button(
                        text("Export Playlistview").align_y(Vertical::Center).align_x(Horizontal::Left)
                    ).padding([4, 8])
                    .width(Length::Fill)
                    .style(button::secondary))
            )).spacing(5.0))
            (button(
                text("SongWindow").align_y(Vertical::Center)
            ).padding([4, 8]).style(button::secondary),
            menu_tpl_1(
                menu_items!(
                    (checkbox(
                         "Show Thumbnails", true
                        ).spacing(5.0)
                        .width(Length::Fill)
                        .style(checkbox::secondary))

                    (checkbox(
                         "Show Next Dance", true
                        ).spacing(5.0)
                        .width(Length::Fill)
                        .style(checkbox::secondary))

                     (button(
                        text("Refresh").align_y(Vertical::Center).align_x(Horizontal::Left)
                    ).padding([4, 8]).on_press(Message::Refresh)
                    .width(Length::Fill)
                    .style(button::secondary))
            )).spacing(5.0))
        ).spacing(5.0);

        mb.into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

impl ConfigWindow {}