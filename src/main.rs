mod dataloading;
mod macros;
mod ui;

use crate::dataloading::dataprovider::song_data_provider::{
    SongChange, SongDataProvider, SongDataSource,
};
use crate::dataloading::id3tagreader::read_song_info_from_filepath;
use crate::dataloading::m3uloader::load_tag_data_from_m3u;
use crate::ui::config_window::ConfigWindow;
use crate::ui::song_window::SongWindow;
use iced::keyboard::key::Named;
use iced::keyboard::{on_key_press, Key, Modifiers};
use iced::widget::horizontal_space;
use iced::{exit, window, Element, Size, Subscription, Task, Theme};
use rfd::FileDialog;
use std::path::PathBuf;

fn main() -> iced::Result {
    iced::daemon(
        DanceInterpreter::title,
        DanceInterpreter::update,
        DanceInterpreter::view,
    )
    .theme(DanceInterpreter::theme)
    .subscription(DanceInterpreter::subscription)
    .run_with(DanceInterpreter::new)
}

pub trait Window {
    fn on_create(&mut self, id: window::Id);
    fn on_resize(&mut self, size: Size);
}

#[derive(Default)]
struct DanceInterpreter {
    config_window: ConfigWindow,
    song_window: SongWindow,

    data_provider: SongDataProvider,
}

#[derive(Debug, Clone)]
pub enum Message {
    Noop,

    WindowOpened(window::Id),
    WindowResized((window::Id, Size)),
    WindowClosed(window::Id),

    ToggleFullscreen,
    SetFullscreen(bool),

    OpenPlaylist,
    FileDropped(PathBuf),
    SongChanged(SongChange),

    EnableImage(bool),
    EnableNextDance(bool),
}

impl DanceInterpreter {
    pub fn new() -> (Self, Task<Message>) {
        let mut tasks = Vec::new();

        let (config_window, cw_opened) = Self::open_window(window::Settings {
            platform_specific: Self::get_platform_specific(),
            ..Default::default()
        });
        let (song_window, sw_opened) = Self::open_window(window::Settings {
            platform_specific: Self::get_platform_specific(),
            ..Default::default()
        });

        let state = Self {
            config_window,
            song_window,

            ..Self::default()
        };

        tasks.push(cw_opened);
        tasks.push(sw_opened);

        (state, Task::batch(tasks))
    }

    fn open_window<T: Window + Default>(settings: window::Settings) -> (T, Task<Message>) {
        let (id, open) = window::open(settings);

        let mut window = T::default();
        window.on_create(id);

        (window, open.map(Message::WindowOpened))
    }

    fn get_platform_specific() -> window::settings::PlatformSpecific {
        #[cfg(target_os = "linux")]
        return window::settings::PlatformSpecific {
            application_id: "danceinterpreter".to_string(),
            ..Default::default()
        };

        #[cfg(not(target_os = "linux"))]
        return Default::default();
    }

    pub fn title(&self, window_id: window::Id) -> String {
        if self.config_window.id == Some(window_id) {
            "Config Window".to_string()
        } else if self.song_window.id == Some(window_id) {
            "Song Window".to_string()
        } else {
            String::new()
        }
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if self.config_window.id == Some(window_id) {
            self.config_window.view(self)
        } else if self.song_window.id == Some(window_id) {
            self.song_window.view(self)
        } else {
            horizontal_space().into()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(_) => ().into(),
            Message::WindowResized((window_id, size)) => {
                println!("Window resized: {:?}, {:?}", window_id, size);
                if self.config_window.id == Some(window_id) {
                    self.config_window.on_resize(size);
                } else if self.song_window.id == Some(window_id) {
                    self.song_window.on_resize(size);
                }

                ().into()
            }
            Message::WindowClosed(window_id) => {
                if self.config_window.id == Some(window_id) {
                    self.config_window.id = None;

                    match self.song_window.id {
                        Some(window_id) => window::close(window_id),
                        None => exit(),
                    }
                } else if self.song_window.id == Some(window_id) {
                    self.song_window.id = None;

                    match self.config_window.id {
                        Some(window_id) => window::close(window_id),
                        None => exit(),
                    }
                } else {
                    ().into()
                }
            }
            Message::ToggleFullscreen => {
                let Some(song_window_id) = self.song_window.id else {
                    return ().into();
                };

                window::get_mode(song_window_id)
                    .map(|mode| Message::SetFullscreen(mode != window::Mode::Fullscreen))
            }
            Message::SetFullscreen(fullscreen) => {
                let Some(song_window_id) = self.song_window.id else {
                    return ().into();
                };

                window::change_mode(
                    song_window_id,
                    if fullscreen {
                        window::Mode::Fullscreen
                    } else {
                        window::Mode::Windowed
                    },
                )
            }

            Message::OpenPlaylist => {
                // Open playlist file
                let file = FileDialog::new()
                    .add_filter("Playlist", &["m3u", "m3u8"])
                    .add_filter("Any(*)", &["*"])
                    .set_title("Select playlist file")
                    .set_directory(
                        dirs::audio_dir().unwrap_or(dirs::home_dir().unwrap_or(PathBuf::from("."))),
                    )
                    .pick_file();

                let Some(file) = file else {
                    return ().into();
                };
                println!("Selected file: {:?}", file);

                let Ok(playlist) = load_tag_data_from_m3u(&file) else {
                    return ().into();
                };

                self.data_provider.set_vec(playlist);

                ().into()
            }

            Message::FileDropped(path) => {
                if let Ok(playlist) = load_tag_data_from_m3u(&path) {
                    self.data_provider.set_vec(playlist);
                } else if let Ok(song_info) = read_song_info_from_filepath(&path) {
                    self.data_provider
                        .set_source(SongDataSource::Other(song_info));
                }

                ().into()
            }

            Message::SongChanged(song_change) => {
                self.data_provider.handle_song_change(song_change);
                ().into()
            }

            Message::EnableImage(state) => {
                self.song_window.enable_image = state;
                ().into()
            }

            Message::EnableNextDance(state) => {
                self.song_window.enable_next_dance = state;
                ().into()
            }
            _ => ().into(),
        }
    }

    fn theme(&self, window_id: window::Id) -> Theme {
        if self.song_window.id.is_some_and(|id| id == window_id) {
            Theme::Dark
        } else {
            Theme::default()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            window::close_events().map(Message::WindowClosed),
            window::resize_events().map(Message::WindowResized),
            window::events().map(|(_, event)| match event {
                window::Event::FileDropped(path) => Message::FileDropped(path),
                _ => Message::Noop,
            }),
            on_key_press(|key: Key, _modifiers: Modifiers| match key {
                Key::Named(Named::ArrowRight) | Key::Named(Named::Space) => {
                    Some(Message::SongChanged(SongChange::Next))
                }
                Key::Named(Named::ArrowLeft) => Some(Message::SongChanged(SongChange::Previous)),
                Key::Named(Named::End) => Some(Message::SongChanged(SongChange::StaticAbsolute(0))),
                Key::Named(Named::F11) => Some(Message::ToggleFullscreen),
                _ => None,
            }),
        ])
    }
}
