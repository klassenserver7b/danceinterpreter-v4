use crate::model::SongInfo;

pub trait SongViewer {
    fn display_song(&self, song: SongInfo);
}

pub struct ConsoleViewer;

impl SongViewer for ConsoleViewer {
    fn display_song(&self, song: SongInfo) {
        println!("Title: {:?}", song);
    }
}

