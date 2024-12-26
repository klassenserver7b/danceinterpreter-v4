mod dataloading;
mod macros;
mod ui;

use crate::dataloading::m3uloader::load_tag_data_from_m3u;
use crate::ui::config_window::ConfigWindow;
use crate::ui::song_window::SongWindow;
use iced::widget::{horizontal_space, image};
use iced::{exit, window, Element, Size, Subscription, Task, Theme};
use rfd::FileDialog;
use std::path::PathBuf;
use crate::dataloading::songinfo::SongInfo;

fn main() -> iced::Result {
    iced::daemon(DanceInterpreter::title, DanceInterpreter::update, DanceInterpreter::view)
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

    song_info: Option<SongInfo>,
    next_song_info: Option<SongInfo>,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    WindowOpened(window::Id),
    WindowResized((window::Id, Size)),
    WindowClosed(window::Id),

    OpenPlaylist,
    Refresh,
    SongChanged,
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
            song_info: None,
            next_song_info: None,
        };

        tasks.push(cw_opened);
        tasks.push(sw_opened);

        (state, Task::batch(tasks))
    }

    fn open_window<T: Window + Default>(
        settings: window::Settings,
    ) -> (T, Task<Message>) {
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

    pub fn view(&self, window_id: window::Id) -> Element<Message> {
        if self.config_window.id == Some(window_id) {
            self.config_window.view(&self)
        } else if self.song_window.id == Some(window_id) {
            self.song_window.view(&self)
        } else {
            horizontal_space().into()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(_) => {
                ().into()
            }
            Message::WindowResized((window_id, size)) => {
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
                        Some(window_id) => {
                            window::close(window_id)
                        }
                        None => {
                            exit()
                        }
                    }
                } else if self.song_window.id == Some(window_id) {
                    self.song_window.id = None;

                    match self.config_window.id {
                        Some(window_id) => {
                            window::close(window_id)
                        }
                        None => {
                            exit()
                        }
                    }
                } else {
                    ().into()
                }
            }
            Message::OpenPlaylist => {
                // Open playlist file
                let file = FileDialog::new()
                    .add_filter("Playlist", &["m3u", "m3u8"])
                    .add_filter("Any(*)", &["*"])
                    .set_title("Select playlist file")
                    .set_directory(dirs::audio_dir().unwrap_or(dirs::home_dir().unwrap_or(PathBuf::from("."))))
                    .pick_file();

                if file.is_some() {
                    println!("Selected file: {:?}", file);
                    let _playlist = load_tag_data_from_m3u(&file.unwrap());
                }

                ().into()
            }
            Message::Refresh => {
                self.song_info = Some(SongInfo {
                    dance: "Gourmetta".to_owned(),
                    title: "Der DJ aus den Bergen".to_owned(),
                    artist: "DJ Ötzi".to_owned(),
                    album_art: Some(image::Handle::from_path(res_file!("icon.jpg"))),
                    track_number: 0,
                });
                self.next_song_info = Some(SongInfo {
                    dance: "Absturz".to_owned(),
                    ..self.song_info.as_ref().unwrap().clone()
                });

                ().into()
            }
            _ => {
                ().into()
            }
        }
    }

    fn theme(&self, window_id: window::Id) -> Theme {
        if self.song_window.id.is_some_and(|id| id == window_id) {
            Theme::Dark
        } else  {
            Theme::default()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            window::close_events().map(Message::WindowClosed),
            window::resize_events().map(Message::WindowResized),
        ])
    }
}
