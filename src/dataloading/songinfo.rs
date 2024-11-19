use iced::widget::image::Handle;

#[derive(Default, Clone, Debug)]
pub struct SongInfo {
    pub track_number: u32,
    pub title: String,
    pub artist: String,
    pub dance: String,
    pub album_art: Option<Handle>,
}

impl SongInfo {
    pub fn new(
        track_number: u32,
        title: String,
        artist: String,
        dance: String,
        album_art: Option<Handle>,
    ) -> Self {
        SongInfo {
            track_number,
            title,
            artist,
            dance,
            album_art,
        }
    }
}
