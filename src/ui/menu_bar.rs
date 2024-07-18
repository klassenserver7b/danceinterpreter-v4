use gtk::{Application, FileDialog, gio, Switch};
use gtk::gio::{ActionEntry, Menu, SimpleAction};
use gtk::glib::clone;
use gtk::prelude::*;

use crate::dataloading::dataprovider::playlist_dataprovider::PlaylistDataProvider;
use crate::dataloading::m3uloader::load_tag_data_from_m3u;

pub fn create_menubar(app: &Application) {
    let file_menu = Menu::new();
    file_menu.append(Some("Open Playlist"), Some("app.menu_file_open_m3u"));
    file_menu.append(Some("Exit"), Some("app.menu_file_exit"));

    let edit_menu = Menu::new();
    edit_menu.append(
        Some("Enable Playlistview"),
        Some("app.menu_edit_playlistview_switch"),
    );
    edit_menu.append(Some("Export Playlistview"), None);
    edit_menu.append(Some("Import Playlistview"), None);

    let view_menu = Menu::new();
    view_menu.append(
        Some("Show Gif in Config Window"),
        Some("app.menu_view_gif_switch"),
    );
    view_menu.append(
        Some("Enable DarkMode"),
        Some("app.menu_view_darkmode_switch"),
    );

    let songwindow_menu = Menu::new();
    songwindow_menu.append(Some("Refresh"), None);
    songwindow_menu.append(
        Some("Show Thumbnails"),
        Some("app.menu_songwindow_thumbnail_switch"),
    );
    songwindow_menu.append(
        Some("Show Next Dance"),
        Some("app.menu_songwindow_nextdance_switch"),
    );

    let menu = Menu::new();
    menu.append_submenu(Some("File"), &file_menu);
    menu.append_submenu(Some("Edit"), &edit_menu);
    menu.append_submenu(Some("View"), &view_menu);
    menu.append_submenu(Some("Song Window"), &songwindow_menu);

    app.set_menubar(Some(&menu));
}

pub fn register_menu_actions(app: &Application) {
    let file_open_m3u = ActionEntry::builder("menu_file_open_m3u")
        .activate(|app: &Application, _, _| handle_file_open_m3u(app))
        .build();
    let exit = ActionEntry::builder("menu_file_exit")
        .activate(|app: &Application, _, _| app.quit())
        .build();

    app.add_action_entries([file_open_m3u, exit]);
}

pub fn register_actions(app: &Application) {
    app.add_action(&create_switch_action(
        "menu_edit_playlistview_switch",
        handle_edit_playlistview_switch_action,
    ));
    app.add_action(&create_switch_action(
        "menu_view_gif_switch",
        handle_view_gif_switch_action,
    ));
    app.add_action(&create_switch_action(
        "menu_view_darkmode_switch",
        handle_view_darkmode_switch_action,
    ));
    app.add_action(&create_switch_action(
        "menu_songwindow_thumbnail_switch",
        handle_songwindow_thumbnail_switch_action,
    ));
    app.add_action(&create_switch_action(
        "menu_songwindow_nextdance_switch",
        handle_songwindow_nextdance_switch_action,
    ));
}

fn create_switch_action(name: &str, handler: fn(bool)) -> SimpleAction {
    let switch = Switch::new();
    switch.set_state(true);
    let switch_action = SimpleAction::new_stateful(name, None, &false.to_variant());
    switch_action.connect_activate(clone!(
        #[strong]
        switch,
        move |action, _| {
            let mut is_active = false;
            if let Some(g) = action.state() {
                is_active = g.get().expect("couldn't get bool");
                // We update the state of the toggle.
                switch.set_active(!is_active);
            }
            // We need to change the toggle state ourselves. `gio` dark magic.
            action.change_state(&(!is_active).to_variant());

            handler(is_active);
        }
    ));
    switch_action
}

fn handle_file_open_m3u(app: &Application) {
    let m3u_filter = gtk::FileFilter::new();
    m3u_filter.set_name(Some("m3u Playlist File (*.m3u *.m3u8)"));
    m3u_filter.add_mime_type("audio/x-mpegurl");
    m3u_filter.add_suffix("m3u");
    m3u_filter.add_suffix("m3u8");

    let all_filter = gtk::FileFilter::new();
    all_filter.set_name(Some("All Files (*)"));
    all_filter.add_mime_type("application/octet-stream");

    let m3u_select_filters = gio::ListStore::new::<gtk::FileFilter>();
    m3u_select_filters.append(&m3u_filter);
    m3u_select_filters.append(&all_filter);

    FileDialog::builder()
        .title("Select m3u file")
        .accept_label("Load")
        .filters(&m3u_select_filters)
        .build()
        .open(
            Some(&app.windows()[0]),
            gio::Cancellable::NONE,
            move |file| {
                if let Ok(file) = file {
                    let filename = file.path().expect("Couldn't get file path");
                    println!("Selected file: {}", filename.display());
                    let playlist_data_provider: PlaylistDataProvider = PlaylistDataProvider::new(&load_tag_data_from_m3u(&filename).unwrap());
                }
            },
        );
}

fn handle_edit_playlistview_switch_action(state: bool) {}
fn handle_view_gif_switch_action(state: bool) {}
fn handle_view_darkmode_switch_action(state: bool) {}
fn handle_songwindow_thumbnail_switch_action(state: bool) {}
fn handle_songwindow_nextdance_switch_action(state: bool) {}
