use std::path::Path;

use crate::ui::gif_paintable::GifPaintable;
use gtk::gio::{Cancellable, File};
use gtk::{glib, Builder};
use gtk::glib::Propagation::Stop;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Picture};
use crate::model::SongInfo;

#[derive(glib::Downgrade)]
pub struct ConfigWindow {
    pub window: ApplicationWindow,
    pub picture: Picture,
}

impl ConfigWindow {
    pub fn new(app: &Application) -> Self {
        let ui_src = include_str!("config_window.ui");
        let builder = Builder::from_string(ui_src);

        let window = builder
            .object::<gtk::ApplicationWindow>("config_window")
            .expect("Couldn't get window");
        window.set_application(Some(app));

        window.connect_close_request(|win: &ApplicationWindow| {
            if let Some(app) = win.application() {
                app.quit();
            }
            Stop
        });

        let picture = create_gif_picture("./pics/tech_dance1.gif");
        ConfigWindow { window, picture }
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

    pub fn toggle_playlist_view(&self, state: bool, song: &[SongInfo]) {}
}

fn create_gif_picture(path: impl AsRef<Path>) -> Picture {
    let paintable = GifPaintable::default();
    let (bytes, _) = File::for_path(path).load_contents(Cancellable::NONE).unwrap();
    paintable.load_from_bytes(&bytes).unwrap();

    Picture::for_paintable(&paintable)
}