use crate::dataloading::dataprovider::song_data_provider::{
    SongChange, SongDataEdit, SongDataSource,
};
use crate::dataloading::songinfo::SongInfo;
use crate::ui::widget::dynamic_text_input::DynamicTextInput;
use crate::{DanceInterpreter, Message, Window};
use iced::advanced::Widget;
use iced::alignment::Vertical;
use iced::border::Radius;
use iced::widget::{
    button, checkbox, column as col, row, scrollable, text, Button, Column, Row, Scrollable, Space,
};
use iced::{font, window, Border, Color, Element, Font, Length, Renderer, Size, Theme};
use iced_aw::menu::Item;
use iced_aw::style::{menu_bar::primary, Status};
use iced_aw::widget::InnerBounds;
use iced_aw::{menu, menu_bar, menu_items, quad, Menu, MenuBar};

#[derive(Default)]
pub struct ConfigWindow {
    pub id: Option<window::Id>,
    pub size: Size,
}

impl Window for ConfigWindow {
    fn on_create(&mut self, id: window::Id) {
        self.id = Some(id);
    }

    fn on_resize(&mut self, size: Size) {
        self.size = size;
    }
}

impl ConfigWindow {
    pub fn view<'a>(&'a self, dance_interpreter: &'a DanceInterpreter) -> Element<'a, Message> {
        let menu_bar = self.build_menu_bar(dance_interpreter);
        let playlist_view = self.build_playlist_view(dance_interpreter);
        let statics_view = self.build_statics_view(dance_interpreter);

        let content = col![menu_bar, playlist_view, statics_view];
        content.into()
    }

    fn build_playlist_view(&self, dance_interpreter: &DanceInterpreter) -> Column<Message> {
        let trow: Row<_> = row![
            text!("Status").width(Length::Shrink),
            text!("Title").width(Length::Fill),
            text!("Artist").width(Length::Fill),
            text!("Dance").width(Length::Fill),
            Space::new(Length::Fill, Length::Shrink),
            Space::new(Length::Fixed(10.0), Length::Shrink),
        ]
        .spacing(5);

        let mut playlist_column: Column<'_, _, _, _> = col!().spacing(5);

        for (i, song) in dance_interpreter
            .data_provider
            .playlist_songs
            .iter()
            .enumerate()
        {
            let song_row = row![
                text!("Status").width(Length::Shrink),
                DynamicTextInput::<'_, Message>::new("Title", &song.title)
                    .width(Length::Fill)
                    .on_change(move |v| Message::SongDataEdit(i, SongDataEdit::Title(v))),
                DynamicTextInput::<'_, Message>::new("Artist", &song.artist)
                    .width(Length::Fill)
                    .on_change(move |v| Message::SongDataEdit(i, SongDataEdit::Artist(v))),
                DynamicTextInput::<'_, Message>::new("Dance", &song.dance)
                    .width(Length::Fill)
                    .on_change(move |v| Message::SongDataEdit(i, SongDataEdit::Dance(v))),
                row![
                    label_message_button_fill(
                        "Present",
                        Message::SongChanged(SongChange::PlaylistAbsolute(i))
                    ),
                    label_message_button_fill(
                        "Set as next",
                        Message::SetNextSong(SongDataSource::Playlist(i))
                    ),
                ]
                .spacing(5)
                .width(Length::Fill),
            ]
            .spacing(5);

            if !playlist_column.children().is_empty() {
                playlist_column = playlist_column.push(separator());
            }

            playlist_column = playlist_column.push(song_row);
        }

        let playlist_scrollable: Scrollable<'_, Message> = scrollable(playlist_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(5);

        col!(trow, playlist_scrollable).spacing(5)
    }

    fn build_statics_view<'a>(&self, dance_interpreter: &'a DanceInterpreter) -> Row<'a, Message> {
        let bold_font = Font {
            family: font::Family::SansSerif,
            weight: font::Weight::Bold,
            stretch: font::Stretch::Normal,
            style: font::Style::Normal,
        };

        let blankb: Button<Message> =
            button(text("Blank").align_y(Vertical::Center).font(bold_font))
                .style(button::secondary)
                .on_press(Message::SongChanged(SongChange::Blank));
        let mut statics: Vec<Element<_>> = dance_interpreter
            .data_provider
            .statics
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                button(text(s.clone()).font(bold_font))
                    .on_press(Message::SongChanged(SongChange::StaticAbsolute(idx)))
                    .into()
            })
            .collect();

        statics.insert(0, blankb.into());

        row(statics).width(Length::Fill).align_y(Vertical::Bottom)
    }

    fn build_menu_bar<'a>(
        &self,
        dance_interpreter: &'a DanceInterpreter,
    ) -> MenuBar<'a, Message, Theme, Renderer> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(150.0).offset(15.0).spacing(5.0);

        #[rustfmt::skip]
        let mb = menu_bar!
        (
            (
                label_message_button_shrink("File", Message::Noop),
                menu_tpl_1(
                    menu_items!(
                        (label_message_button_fill("Open Playlist File", Message::OpenPlaylist))
                        (label_message_button_fill("Exit", Message::Noop))
                    )
                )
                .spacing(5.0)
            )
            (
                label_message_button_shrink("Edit", Message::Noop),
                menu_tpl_1(
                    menu_items!(
                        (label_message_button_fill("Import Playlistview", Message::Noop))
                        (label_message_button_fill("Export Playlistview", Message::Noop))
                        (label_message_button_fill("Add blank song", Message::AddSong(SongInfo::default())))
                    )
                )
                .spacing(5.0)
            )
            (
                label_message_button_shrink("SongWindow", Message::Noop),
                menu_tpl_1(
                    menu_items!(
                        (labeled_message_checkbox("Show Thumbnails", dance_interpreter.song_window.enable_image, Message::EnableImage))
                        (labeled_message_checkbox("Show Next Dance", dance_interpreter.song_window.enable_next_dance, Message::EnableNextDance))
                        (label_message_button_fill("Refresh", Message::Noop))
                    )
                )
                .spacing(5.0)
            )
        )
        .spacing(5.0)
        .draw_path(menu::DrawPath::Backdrop)
            .style(|theme:&iced::Theme, status: Status | menu::Style{
                path_border: Border{
                    radius: Radius::new(6.0),
                    ..Default::default()
                },
                ..primary(theme, status)
            });

        mb
    }
}

fn label_message_button_fill(label: &str, message: Message) -> button::Button<Message> {
    label_message_button(label, message).width(Length::Fill)
}

fn label_message_button_shrink(label: &str, message: Message) -> button::Button<Message> {
    label_message_button(label, message).width(Length::Shrink)
}

fn label_message_button(label: &str, message: Message) -> button::Button<Message> {
    button(text(label).align_y(Vertical::Center))
        .padding([4, 8])
        .style(button::secondary)
        .on_press(message)
}

fn labeled_message_checkbox(
    label: &str,
    checked: bool,
    message: fn(bool) -> Message,
) -> checkbox::Checkbox<Message> {
    checkbox(label, checked)
        .on_toggle(message)
        .width(Length::Fill)
    //.style(checkbox::secondary)
}

fn separator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(2.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(1.0, 0.2),
        height: Length::Fixed(5.0),
        width: Length::Fill,
        ..Default::default()
    }
}
