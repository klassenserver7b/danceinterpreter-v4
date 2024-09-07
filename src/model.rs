use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Properties;
use gtk::glib::Bytes;
use gtk::glib;
use id3::frame::Picture;
use std::cell::RefCell;
use std::sync::atomic::AtomicU32;

mod imp {
    use super::*;
    use std::sync::atomic::AtomicU32;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::SongInfo)]
    pub struct SongInfo {
        #[property(get, set)]
        track_number: AtomicU32,
        #[property(get, set)]
        title: RefCell<String>,
        #[property(get, set)]
        artist: RefCell<String>,
        #[property(get, set)]
        dance: RefCell<String>,
        #[property(get, set, nullable)]
        album_art: RefCell<Option<Bytes>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SongInfo {
        const NAME: &'static str = "SongInfo";
        type Type = super::SongInfo;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SongInfo {}
}

glib::wrapper! {
    pub struct SongInfo(ObjectSubclass<imp::SongInfo>);
}

impl SongInfo {
    pub fn new(track_number: u32, title: String, artist: String, dance: String, album_art: Option<Picture>) -> Self {
        let song_info: SongInfo = glib::object::Object::new();

        song_info.set_track_number(track_number);
        song_info.set_title(title);
        song_info.set_artist(artist);
        song_info.set_dance(dance);
        song_info.set_album_art(album_art.map(|p| Bytes::from(&p.data)));

        song_info
    }
}
