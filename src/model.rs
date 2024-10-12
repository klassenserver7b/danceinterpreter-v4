use id3::frame::Picture;
use std::cell::RefCell;
use std::sync::atomic::AtomicU32;
use crate::model::imp::SongInfo;

mod imp {
    use super::*;
    use std::sync::atomic::AtomicU32;
    use bytes::Bytes;

    #[derive(Default)]
    pub struct SongInfo {
        track_number: AtomicU32,
        title: RefCell<String>,
        artist: RefCell<String>,
        dance: RefCell<String>,
        album_art: RefCell<Option<Bytes>>,
    }
}
