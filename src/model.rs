#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SongInfo {
    pub title: String,
    pub artist: String,
    pub dance: String,
    pub album_art: (), // TODO: image type
}