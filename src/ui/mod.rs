use crate::model::SongInfo;

pub mod ui_manager;
pub mod config_window;
pub mod song_window;
pub mod menu_bar;
pub mod gif_paintable;
pub mod switch_action;
pub mod actions;

pub trait SongViewer {
    fn display_song(&self, song: SongInfo);
}

pub struct ConsoleViewer;

impl SongViewer for ConsoleViewer {
    fn display_song(&self, song: SongInfo) {
        println!("Title: {:?}", song);
    }
}

