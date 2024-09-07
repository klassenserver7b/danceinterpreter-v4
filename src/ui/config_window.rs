use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::model::SongInfo;
use crate::ui::gif_paintable::GifPaintable;
use gtk::gdk::{ContentFormats, DragAction, FileList};
use gtk::gio::{Cancellable, File, ListStore};
use gtk::glib::clone;
use gtk::glib::Propagation::Stop;
use gtk::prelude::*;
use gtk::{glib, Builder, ColumnView, DropTarget};
use gtk::{Application, ApplicationWindow, Picture};
use crate::dataloading::id3tagreader::{read_song_info_from_file, read_song_info_from_filepath, read_song_info_from_files};

#[derive(glib::Downgrade)]
pub struct ConfigWindow {
    pub window: ApplicationWindow,
    pub picture: Picture,
    pub ls_tracks: ListStore,
}

impl ConfigWindow {
    pub fn new(app: &Application) -> Self {
        let ui_src = include_str!("config_window.ui");
        let builder = Builder::from_string(ui_src);

        let window = builder
            .object::<gtk::ApplicationWindow>("config_window")
            .expect("Couldn't load config window");
        window.set_application(Some(app));

        window.connect_close_request(|win: &ApplicationWindow| {
            if let Some(app) = win.application() {
                app.quit();
            }
            Stop
        });

        let ls_tracks = builder
            .object::<ListStore>("ls_tracks")
            .expect("Couldn't load config window track list store");

        ls_tracks.append(&SongInfo::new(
            0,
            "Der DJ aus den Bergen".to_string(),
            "DJ Ötzi".to_string(),
            "brain damage".to_string(),
            None,
        ));

        let target = DropTarget::builder().name("file-drop-target").actions(DragAction::COPY)
            .formats(
                &ContentFormats::builder()
                    .add_type(FileList::static_type())
                    .add_type(File::static_type())
                    .add_type(glib::types::Type::STRING)
                    .add_mime_type("audio/mpeg")
                    .build()
            ).build();

        target.connect_drop(
            clone!(
            #[weak]
            ls_tracks,
                #[upgrade_or]
                false,
                move |_, value, _, _| {
                    if value.is::<FileList>() {
                        let file_list = value.get::<FileList>();
                        let song_infos = read_song_info_from_files(&file_list.unwrap());
                        println!("{:?}", song_infos);
                        true
                    } else if value.is::<File>() {
                        let file = value.get::<File>();
                        let song_info = read_song_info_from_file(&file.unwrap());
                        println!("{:?}", song_info);
                        true
                    } else if value.is::<String>() {
                        let file_path = value.get::<String>();
                        let song_info = read_song_info_from_filepath(&PathBuf::from_str(&file_path.unwrap()).unwrap());
                        println!("{:?}", song_info);
                        true
                    }
                    else { false }
                }
        ));

        let cv_tracks = builder
            .object::<ColumnView>("tracks_view")
            .expect("Couldn't load config window track view");

        cv_tracks.add_controller(target);

        let picture = create_gif_picture("./pics/tech_dance1.gif");
        ConfigWindow { window, picture, ls_tracks }
    }

    pub fn present(&self) {
        self.window.present();
    }

    pub fn set_gif(&self, gif: &GifPaintable) {
        self.picture.set_paintable(Some(gif));
    }

    pub fn set_gif_state(&self, state: bool) {
        if state {
            self.window.set_child(Some(&self.picture));
        } else {
            self.window.set_child(None as Option::<&gtk::Widget>);
        }
    }

    pub fn toggle_playlist_view(&self, _state: bool, _song: &[SongInfo]) {}

    pub fn set_song_list(&self, song_list: &Vec<SongInfo>) {
        self.ls_tracks.remove_all();

        for song_info in song_list {
            self.ls_tracks.append(song_info);
        }
    }
}

fn create_gif_picture(path: impl AsRef<Path>) -> Picture {
    let paintable = GifPaintable::default();
    let (bytes, _) = File::for_path(path).load_contents(Cancellable::NONE).unwrap();
    paintable.load_from_bytes(&bytes).unwrap();

    Picture::for_paintable(&paintable)
}