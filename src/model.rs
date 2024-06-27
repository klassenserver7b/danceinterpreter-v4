use id3::Content;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SongInfo  {
    pub title: String,
    pub artist: String,
    pub dance: String,
    pub album_art: Option<Content>,
}

impl SongInfo{
    pub fn new(title: String, artist: String, dance: String, album_art: Option<Content>) -> Self {
        Self {
            title,
            artist,
            dance,
            album_art,
        }
    }
}