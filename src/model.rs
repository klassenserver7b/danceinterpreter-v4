use id3::frame::Picture;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SongInfo {
    pub title: String,
    pub artist: String,
    pub dance: String,
    pub album_art: Option<Picture>,
}

impl SongInfo {
    pub fn new(title: String, artist: String, dance: String, album_art: Option<Picture>) -> Self {
        Self {
            title,
            artist,
            dance,
            album_art,
        }
    }
}