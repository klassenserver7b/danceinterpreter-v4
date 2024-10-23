use iced_aw::menu::Item;
use crate::{Message, Window};
use iced::widget::{button, horizontal_space, text};
use iced::{alignment, window, Element, Length, Size, Task, Theme};
use iced_aw::{menu_bar, menu_items, Menu};

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

        ().into()
    }

    fn view(&self) -> Element<Message> {
        let menu_tpl_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let mb = menu_bar!(
            (button(
                text("File").align_y(alignment::Vertical::Center)
            ).padding([4, 8]).style(button::primary),
            menu_tpl_1(
                menu_items!(
                (text("Open").align_y(alignment::Vertical::Center))
                (button(
                        text("Exit").align_y(alignment::Vertical::Center)
                    ).padding([4, 8]).style(button::primary))
            ))));

        mb.into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

impl ConfigWindow {}