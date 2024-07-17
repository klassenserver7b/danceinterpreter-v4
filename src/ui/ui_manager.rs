use gtk::{Application, Window};
use gtk::prelude::GtkApplicationExt;

pub fn get_config_window(app: &Application) -> Window {
    app.window_by_id(0).unwrap()
}
pub fn get_song_window(app: &Application) -> Window {
    app.window_by_id(1).unwrap()
}