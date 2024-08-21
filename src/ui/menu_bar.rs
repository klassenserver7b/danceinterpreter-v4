use gtk::prelude::*;
use gtk::{Application, Builder};

pub fn create_menubar(app: &Application) {
    let menu_src = include_str!("menu_bar.ui");
    let builder = Builder::from_string(menu_src);
    
    let menu = builder
        .object::<gtk::gio::Menu>("menu_bar")
        .expect("Couldn't get menu bar");

    app.set_menubar(Some(&menu));
}
