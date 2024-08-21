use std::cell::RefCell;
use std::rc::Rc;

use danceinterpreter_v4::dataloading::dataprovider::playlist_dataprovider::PlaylistDataProvider;
use danceinterpreter_v4::ui::actions::register_actions;
use danceinterpreter_v4::ui::menu_bar::create_menubar;
use danceinterpreter_v4::ui::ui_manager::{create_shortcuts, load_css, UIManager};
use gtk::prelude::*;
use gtk::Application;

fn main() {
    let app = Application::builder()
        .application_id("de.klassenserver7b.DanceInterpreter")
        .build();

    app.connect_startup(|app| {
        create_menubar(app);
        load_css();
    });

    app.connect_activate(|app: &Application| {
        let ui_manager = Rc::new(UIManager::new(app));
        let data_provider = Rc::new(RefCell::new(PlaylistDataProvider::new(Vec::default(), ui_manager.clone())));

        ui_manager.get_config_window().present();
        register_actions(app, data_provider);
        create_shortcuts(app);
    });

    app.run();
}
