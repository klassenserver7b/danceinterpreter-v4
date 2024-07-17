use gtk::{Application, ApplicationWindow, Picture};
use gtk::gio::{Cancellable, File};
use gtk::prelude::{FileExtManual, GtkWindowExt};

use crate::ui::gif_paintable::GifPaintable;

pub fn create_config_window(app: &Application) -> ApplicationWindow {
    let win = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .startup_id("config_window")
        .show_menubar(true)
        .build();

    let paintable = GifPaintable::default();
    let (bytes, _) = File::for_path("./pics/tech_dance1.gif").load_contents(Cancellable::NONE).unwrap();
    paintable.load_from_bytes(&bytes).unwrap();
    let picture = Picture::for_paintable(&paintable);

    win.set_child(Some(&picture));
    win.present();
    win
}