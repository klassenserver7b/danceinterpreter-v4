use std::cmp::PartialEq;
use crate::dataloading::songinfo::SongInfo;


#[derive(Default, PartialEq)]
pub enum SongDataSource {
    #[default]
    Blank,
    Other(SongInfo),
    Static(usize),
    Playlist(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum SongChange {
    Blank,
    StaticAbsolute(usize),
    PlaylistAbsolute(usize),
    Previous,
    Next,
}

#[derive(Default)]
pub struct SongDataProvider
{
    pub source: SongDataSource,
    pub playlist_songs: Vec<SongInfo>,
    pub statics: Vec<String>,
}

impl SongDataProvider {
    pub fn set_vec(&mut self, vec: Vec<SongInfo>) {
        self.playlist_songs = vec;

        if !self.playlist_songs.is_empty() {
            self.source = SongDataSource::Playlist(0);
        } else {
            self.source = SongDataSource::Blank;
        }
    }

    pub fn get_current_song_info(&self) -> Option<&SongInfo> {
        match self.source {
            SongDataSource::Static(_) => None, //TODO: return static song info
            SongDataSource::Playlist(i) => self.playlist_songs.get(i),
            SongDataSource::Other(ref song) => Some(song),
            SongDataSource::Blank => None,
        }
    }
    pub fn get_next_song_info(&self) -> Option<&SongInfo> {
        match self.source {
            SongDataSource::Static(_) => None,
            SongDataSource::Playlist(i) => self.playlist_songs.get(i + 1),
            SongDataSource::Blank => None,
            SongDataSource::Other(_) => None,
        }
    }

    pub fn prev(&mut self) {
        let SongDataSource::Playlist(current_index) = self.source else {
            return;
        };

        if current_index == 0 {
            return;
        }

        self.source = SongDataSource::Playlist(current_index - 1);
    }

    pub fn next(&mut self) {
        let SongDataSource::Playlist(current_index) = self.source else {
            return;
        };

        if current_index == self.playlist_songs.len() - 1 {
            return;
        }

        self.source = SongDataSource::Playlist(current_index + 1);
    }

    pub fn set_source(&mut self, n: SongDataSource) {
        match n {
            SongDataSource::Static(i) => {
                if self.playlist_songs.get(i).is_some() {
                    self.source = n;
                }
            }
            SongDataSource::Playlist(i) => {
                if self.playlist_songs.get(i).is_some() {
                    self.source = n;
                }
            }
            _ => self.source = n,
        }
    }

    pub fn handle_song_change(&mut self, change: SongChange) {
        match change {
            SongChange::Blank => {
                self.source = SongDataSource::Blank;
            }
            SongChange::StaticAbsolute(index) => {
                self.source = SongDataSource::Static(index);
            }
            SongChange::PlaylistAbsolute(index) => {
                self.source = SongDataSource::Playlist(index);
            }
            SongChange::Previous => {
                self.prev();
            }
            SongChange::Next => {
                self.next();
            }
        }
    }
}
