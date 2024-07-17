use gtk::Application;
use gtk::prelude::*;

use danceinterpreter_v4::ui::config_window::create_config_window;
use danceinterpreter_v4::ui::menu_bar::{create_menubar, register_actions, register_menu_actions};

fn main() {
    let app = Application::builder()
        .application_id("de.klassenserver7b.danceinterpreter")
        .build();

    app.connect_startup(|app| {
        register_menu_actions(app);
        register_actions(app);
        create_menubar(app);
    });

    app.connect_activate(|app: &Application| {
        create_config_window(app);
    });

    app.run();
}
