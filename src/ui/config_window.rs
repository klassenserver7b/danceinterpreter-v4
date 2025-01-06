use crate::{DanceInterpreter, Message, Window};
use iced::alignment::Vertical;
use iced::border::Radius;
use iced::widget::{button, checkbox, column as col, container, row, text, Button, Row};
use iced::{font, window, Border, Element, Font, Length, Renderer, Size, Theme};
use iced_aw::menu::Item;
use iced_aw::style::{menu_bar::primary, Status};
use iced_aw::{menu, menu_bar, menu_items, Menu, MenuBar};

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
    pub fn view(&self, dance_interpreter: &DanceInterpreter) -> Element<'_, Message> {

        let menu_bar = self.build_menu_bar(dance_interpreter);
        //let playlist_view = self.build_playlist_view();
        let statics_view = self.build_statics_view(dance_interpreter);

        let content = col![menu_bar, statics_view];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn build_playlist_view(&self){

    }

    fn build_statics_view(&self, dance_interpreter: &DanceInterpreter) -> Row<'_, Message> {
        let bold_font = Font {
            family: font::Family::SansSerif,
            weight: font::Weight::Bold,
            stretch: font::Stretch::Normal,
            style: font::Style::Normal,
        };
        

        let boldb: Button<Message, Theme, Renderer>  = button(text("Blank").font(bold_font)).on_press(Message::Noop);
        let mut statics: Vec<Element<_>> = dance_interpreter.data_provider.statics.iter().map(
            |s| button(text(s.clone()).font(bold_font)).on_press(Message::Noop).into()
        ).collect();

        statics.insert(0, boldb.into());

        row(statics)
    }

    fn build_menu_bar(&self, dance_interpreter: &DanceInterpreter) -> MenuBar<Message, Theme, Renderer>{
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

fn label_message_button_fill(
    label: &str,
    message: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    label_message_button(label, message).width(Length::Fill)
}

fn label_message_button_shrink(
    label: &str,
    message: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    label_message_button(label, message).width(Length::Shrink)
}

fn label_message_button(
    label: &str,
    message: Message,
) -> button::Button<Message, iced::Theme, iced::Renderer> {
    button(text(label).align_y(Vertical::Center))
        .padding([4, 8])
        .style(button::secondary)
        .on_press(message)
}

fn labeled_message_checkbox(
    label: &str,
    checked: bool,
    message: fn(bool) -> Message,
) -> checkbox::Checkbox<Message, iced::Theme, iced::Renderer> {
    checkbox(label, checked)
        .on_toggle(message)
        .width(Length::Fill)
        //.style(checkbox::secondary)
}
