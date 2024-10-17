use iced::{window, Element, Size, Task};
use crate::{Message, Window};

#[derive(Default)]
pub struct ConfigWindow {
    id: Option<window::Id>,
    size: Size,
}

impl Window for ConfigWindow{
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
        todo!()
    }
}