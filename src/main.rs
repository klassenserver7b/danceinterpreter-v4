use gtk::{Application, ApplicationWindow, gio, Picture};
use gtk::prelude::*;

use danceinterpreter_v4::ui::gif_paintable::GifPaintable;
use danceinterpreter_v4::ui::menu_bar::{create_menubar, register_actions, register_menu_actions};

fn main() {
    let app = Application::builder()
        .application_id("de.klassenserver7b.danceinterpreter.kein_plan_.dummer_hs")
        .build();
    
    app.connect_startup(|app| {
        register_menu_actions(app);
        register_actions(app);
        create_menubar(app);
    });

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("DanceInterpreter")
            .show_menubar(true)
            .build();

        let paintable = GifPaintable::default();
        let (bytes, _) = gio::File::for_path("./pics/tech_dance1.gif").load_contents(gio::Cancellable::NONE).unwrap();
        paintable.load_from_bytes(&bytes).unwrap();
        let picture = Picture::for_paintable(&paintable);

        win.set_child(Some(&picture));
        win.present();
    });

    app.run();
}