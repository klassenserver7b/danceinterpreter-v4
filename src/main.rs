mod dataloading;
mod macros;
mod ui;

use crate::ui::config_window::ConfigWindow;
use crate::ui::song_window::SongWindow;
use iced::widget::horizontal_space;
use iced::{window, Element, Size, Subscription, Task, Theme};
use std::collections::BTreeMap;

fn main() -> iced::Result {
    iced::daemon(Counter::title, Counter::update, Counter::view)
        .theme(Counter::theme)
        .subscription(Counter::subscription)
        .run_with(Counter::new)
}

pub trait Window {
    fn set_id(&mut self, id: window::Id);
    fn title(&self) -> String;
    fn update(&mut self, message: Message) -> Task<Message>;
    fn view(&self) -> Element<Message>;
    fn theme(&self) -> Theme;
}

#[derive(Default)]
struct Counter {
    windows: BTreeMap<window::Id, Box<dyn Window>>,
    //global_state: GlobalState,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    WindowOpened(window::Id),
    WindowResized((window::Id, Size)),
    OpenPlaylist,
    Refresh,
    SongChanged,
}

impl Counter {
    pub fn new() -> (Self, Task<Message>) {
        let mut tasks = Vec::new();
        let mut state = Counter {
            windows: BTreeMap::new(),
        };

        tasks.push(
            state.create_window(
                Box::new(SongWindow::default()),
                window::Settings::default()),
        );
        tasks.push(
            state.create_window(
                Box::new(ConfigWindow::default()),
                window::Settings::default(),
        ));

        (state, Task::batch(tasks))
    }

    fn create_window(
        &mut self,
        mut window: Box<dyn Window>,
        settings: window::Settings,
    ) -> Task<Message> {
        let (id, open) = window::open(settings);
        window.set_id(id);
        self.windows.insert(id, window);

        open.map(Message::WindowOpened)
    }

    pub fn title(&self, window_id: window::Id) -> String {
        self.windows
            .get(&window_id)
            .map(|w| w.title())
            .unwrap_or_default()
    }

    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        self.windows
            .get(&window_id)
            .map(|w| w.view())
            .unwrap_or_else(|| horizontal_space().into())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let window_tasks: Vec<Task<Message>> = self
            .windows
            .iter_mut()
            .map(|(_, w)| w.update(message))
            .collect();

        Task::batch(window_tasks)
    }

    fn theme(&self, window_id: window::Id) -> Theme {
        self.windows
            .get(&window_id)
            .map(|w| w.theme())
            .unwrap_or(Theme::Dark)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        window::resize_events().map(Message::WindowResized)
    }
}
