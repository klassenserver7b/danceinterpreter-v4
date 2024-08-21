use crate::model::SongInfo;
use gtk::gdk::{Display, Texture};
use gtk::glib::Propagation::Stop;
use gtk::glib::{closure_local, Bytes, ParamSpec};
use gtk::prelude::{ActionGroupExt, GtkWindowExt, ObjectExt, WidgetExt};
use gtk::{glib, Align, Builder, Picture};
use gtk::{Application, ApplicationWindow, Label, STYLE_PROVIDER_PRIORITY_APPLICATION};
use std::time::Duration;

#[derive(glib::Downgrade)]
pub struct SongWindow {
    pub window: ApplicationWindow,
    information_widgets: InformationWidgets,
}

impl SongWindow {
    pub fn new(app: &Application) -> Self {
        let ui_src = include_str!("song_window.ui");
        let builder = Builder::from_string(ui_src);

        let window = builder
            .object::<gtk::ApplicationWindow>("song_window")
            .expect("Couldn't get window");
        window.set_application(Some(app));

        register_font_style(&window);
        let information_widgets = find_widgets(&builder);

        window.connect_close_request(|win: &ApplicationWindow| {
            if let Some(app) = win.application() {
                app.activate_action("menu_songwindow_show_switch", None);
            }
            Stop
        });

        SongWindow {
            window,
            information_widgets,
        }
    }

    pub fn set_song_info(&self, song_info: &SongInfo, next_dance: &str) {
        self.information_widgets.dance_label.set_text(&song_info.dance);
        self.information_widgets.title_label.set_text(&song_info.title);
        self.information_widgets.artist_label.set_text(&song_info.artist);

        if let Some(picture_data) = &song_info.album_art {
            self.information_widgets.cover_picture.set_paintable(
                Some(&Texture::from_bytes(&Bytes::from(&picture_data.data)).unwrap())
            );
        } else {
            self.information_widgets.cover_picture.set_paintable(None as Option<&Texture>);
        }

        if next_dance.is_empty() {
            self.information_widgets.next_dance_label.set_visible(false);
        } else {
            self.information_widgets.next_dance_label.set_text(next_dance);
            self.information_widgets.next_dance_label.set_visible(true);
        }
    }

    pub fn present(&self) {
        self.window.present();
    }

    pub fn hide(&self) {
        self.window.set_visible(false)
    }
}

fn register_font_style(win: &ApplicationWindow) {
    win.add_css_class("song_window");
    let win_provider = gtk::CssProvider::new();
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &win_provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let update_css = closure_local!(
        #[weak]
        win,
        #[strong]
        win_provider,
        move || {
            let height = win.height() as f32;
            let css = format!(".song_window {{ font-size: {}px; }}", height * 0.01f32);
            win_provider.load_from_string(css.as_str());
    });

    let queue_update_css = closure_local!(
        move |_: &ApplicationWindow, _: &ParamSpec| {
            let update_css_clone = update_css.clone();
            glib::timeout_add_local_once(Duration::from_millis(10), move || update_css_clone.invoke::<()>(&[]));
    });

    win.connect_closure("notify::maximized", true, queue_update_css.clone());
    win.connect_closure("notify::fullscreened", true, queue_update_css.clone());
    win.connect_closure("notify::default-height", true, queue_update_css.clone());
}

fn find_widgets(builder: &Builder) -> InformationWidgets {
    let dance_label = builder
        .object::<gtk::Label>("label_dance")
        .expect("Couldn't get dance label");
    let title_label = builder
        .object::<gtk::Label>("label_title")
        .expect("Couldn't get title label");
    let artist_label = builder
        .object::<gtk::Label>("label_artist")
        .expect("Couldn't get artist label");
    let next_dance_label = builder
        .object::<gtk::Label>("label_next_dance")
        .expect("Couldn't get next dance label");

    let cover_picture = builder
        .object::<gtk::Picture>("picture_cover")
        .expect("Couldn't get cover picture");

    InformationWidgets::new(dance_label, title_label, artist_label, next_dance_label, cover_picture)
}

#[derive(glib::Downgrade)]
pub struct InformationWidgets {
    dance_label: Label,
    title_label: Label,
    artist_label: Label,
    next_dance_label: Label,
    cover_picture: Picture,
}

impl InformationWidgets {
    pub fn new(dance_label: Label, title_label: Label, artist_label: Label, next_dance_label: Label, cover_picture: Picture) -> Self {
        InformationWidgets {
            dance_label,
            title_label,
            artist_label,
            next_dance_label,
            cover_picture,
        }
    }
}