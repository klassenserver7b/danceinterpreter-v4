use crate::model::SongInfo;
use crate::ui::config_window::ConfigWindow;
use crate::ui::song_window::SongWindow;
use gtk::gdk::Display;
use gtk::prelude::GtkApplicationExt;
use gtk::{Application, CssProvider};

pub struct UIManager {
    config_window: ConfigWindow,
    song_window: SongWindow,
}

impl UIManager {
    pub fn new(app: &Application) -> Self {
        let config_window = ConfigWindow::new(app);
        let song_window = SongWindow::new(app);
        UIManager { config_window, song_window }
    }

    pub fn present(&self) {
        self.config_window.present();
        self.song_window.present();
    }
    
    pub fn set_song_info(&self, song_info: &SongInfo, next_dance: &str) {
        self.song_window.set_song_info(song_info, next_dance);
    }
    
    pub fn set_song_list(&self, song_list: &Vec<SongInfo>) {
        self.config_window.set_song_list(song_list);
    }

    pub fn get_config_window(&self) -> &ConfigWindow {
        &self.config_window
    }
    pub fn get_song_window(&self) -> &SongWindow {
        &self.song_window
    }
}

pub fn create_shortcuts(app: &Application) {
    app.set_accels_for_action("app.playlist-dataprovider_next", &["space", "Right", "Page_Up"]);
    app.set_accels_for_action("app.playlist-dataprovider_previous", &["Left", "Page_Down"]);
    app.set_accels_for_action("app.menu_file_test", &["F1"]);
}

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("../../resources/main/style.css"));
    
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}