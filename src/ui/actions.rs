use crate::dataloading::dataprovider::playlist_dataprovider::PlaylistDataProvider;
use crate::dataloading::m3uloader::load_tag_data_from_m3u;
use crate::ui::config_window::ConfigWindow;
use crate::ui::switch_action::SwitchAction;
use crate::ui::ui_manager::UIManager;
use gtk::gio::{ActionEntry, SimpleAction};
use gtk::prelude::{ActionExt, ActionMapExt, ActionMapExtManual, ApplicationExt, FileExt, GtkApplicationExt, ToVariant, WidgetExt};
use gtk::{gio, glib, Application, FileDialog, Switch};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_actions(app: &Application, data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    register_menu_actions(app, data_provider.clone());
    register_switchactions(app, data_provider.clone());
    register_shortcutactions(app, data_provider.clone());
}

fn register_menu_actions(app: &Application, data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    let file_open_m3u = ActionEntry::builder("menu_file_open_m3u")
        .activate(move |app: &Application, _, _| handle_file_open_m3u(app, data_provider.clone()))
        .build();

    let exit = ActionEntry::builder("menu_file_exit")
        .activate(|app: &Application, _, _| app.quit())
        .build();

    let test = ActionEntry::builder("menu_file_test")
        .activate(|_, _, _| {}).build();

    app.add_action_entries([file_open_m3u, exit, test]);
}

fn register_shortcutactions(app: &Application, data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    let data_provider_clone = data_provider.clone();
    app.add_action(&create_shortcut_action("playlist-dataprovider_next", move || {
        data_provider_clone.borrow_mut().next();
    }));

    let data_provider_clone = data_provider.clone();
    app.add_action(&create_shortcut_action("playlist-dataprovider_previous", move || {
        data_provider_clone.borrow_mut().prev();
    }));
}

fn register_switchactions(app: &Application, data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    let data_prov = data_provider.clone();

    app.add_action(&create_switch_action(
        "menu_edit_playlistview_switch", false,
        move |state| {
            handle_edit_playlistview_switch_action(state, data_prov.clone())
        },
    ));


    let ui_man = data_provider.borrow().get_ui_manager();
    app.add_action(&create_switch_action(
        "menu_view_gif_switch", false,
        move |state| {
            handle_view_gif_switch_action(state, ui_man.get_config_window())
        },
    ));
    app.add_action(&create_switch_action(
        "menu_view_darkmode_switch", true,
        handle_view_darkmode_switch_action,
    ));


    let ui_man = data_provider.borrow().get_ui_manager();
    app.add_action(&create_switch_action(
        "menu_songwindow_show_switch", false,
        move |state| {
            handle_songwindow_show_switch_action(state, ui_man.clone());
        }));
    app.add_action(&create_switch_action(
        "menu_songwindow_thumbnail_switch", true,
        handle_songwindow_thumbnail_switch_action,
    ));
    app.add_action(&create_switch_action(
        "menu_songwindow_nextdance_switch", true,
        handle_songwindow_nextdance_switch_action,
    ));
}

fn create_shortcut_action<F: Fn() + 'static>(name: &str, handler: F) -> SimpleAction {
    let action = SimpleAction::new(name, None);
    action.connect_activate(move |_, _| handler());
    action
}

fn create_switch_action<F: Fn(bool) + 'static>(name: &str, state: bool, handler: F) -> SwitchAction {
    let switch = Switch::new();
    switch.set_state(state);
    let switch_action = SwitchAction::new_stateful(name, None, &state.to_variant());
    switch_action.connect_activate(glib::clone!(
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

            handler(!is_active);
        }
    ));
    switch_action
}

fn handle_file_open_m3u(app: &Application, data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    let m3u_filter = gtk::FileFilter::new();
    m3u_filter.set_name(Some("m3u Playlist File (*.m3u *.m3u8)"));
    m3u_filter.add_mime_type("audio/x-mpegurl");
    m3u_filter.add_suffix("m3u");
    m3u_filter.add_suffix("m3u8");

    let all_filter = gtk::FileFilter::new();
    all_filter.set_name(Some("All Files (*)"));
    all_filter.add_suffix("*");

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

                    data_provider.borrow_mut()
                        .set_vec(load_tag_data_from_m3u(&filename).unwrap());
                }
            },
        );
}

fn handle_edit_playlistview_switch_action(_state: bool, _data_provider: Rc<RefCell<PlaylistDataProvider>>) {
    
    
    
}
fn handle_view_gif_switch_action(state: bool, config_window: &ConfigWindow) {
    config_window.set_gif_state(state);
}

fn handle_view_darkmode_switch_action(_state: bool) {}

fn handle_songwindow_show_switch_action(state: bool, ui_manager: Rc<UIManager>) {
    if state {
        ui_manager.get_song_window().present();
    } else {
        ui_manager.get_song_window().window.set_visible(false);
    }
}
fn handle_songwindow_thumbnail_switch_action(_state: bool) {}
fn handle_songwindow_nextdance_switch_action(_state: bool) {}