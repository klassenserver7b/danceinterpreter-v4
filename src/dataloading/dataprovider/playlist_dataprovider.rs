use crate::model::SongInfo;
use crate::ui::ui_manager::UIManager;
use std::rc::Rc;

pub struct PlaylistDataProvider
{
    pub index: usize,
    pub vec: Vec<SongInfo>,

    empty_song_info: SongInfo,
    pub ui_manager: Rc<UIManager>,
}

impl PlaylistDataProvider {
    pub fn new(vec: Vec<SongInfo>, ui_manager: Rc<UIManager>) -> Self {
        Self {
            vec,
            index: 0,
            empty_song_info: SongInfo {
                title: String::new(),
                artist: String::new(),
                dance: String::new(),
                album_art: None,
            },
            ui_manager,
        }
    }

    pub fn set_vec(&mut self, vec: Vec<SongInfo>) {
        self.vec = vec;
        self.index = 0;
        self.provide_data();
    }

    pub fn provide_data(&self) {
        let current_song = self.vec.get(self.index)
            .unwrap_or(&self.empty_song_info);

        let next_dance = self.vec.get(self.index + 1)
            .map(|next_info| next_info.dance.as_str())
            .unwrap_or("");

        self.ui_manager.set_song_info(current_song, next_dance);
    }

    pub fn current(&self) -> Option<&SongInfo> {
        self.vec.get(self.index)
    }

    pub fn next(&mut self) {
        if self.vec.get(self.index + 1).is_some() {
            self.index += 1;
            self.provide_data();
        }
    }

    pub fn set_index(&mut self, n: usize) {
        if self.vec.get(n).is_some() {
            self.index = n;
            self.provide_data();
        }
    }

    pub fn prev(&mut self) {
        if self.index == 0 {
            return;
        }

        self.index -= 1;
        self.provide_data();
    }

    pub fn get_ui_manager(&self) -> Rc<UIManager> {
        self.ui_manager.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::dataloading::dataprovider::playlist_dataprovider::PlaylistDataProvider;
    use crate::model::SongInfo;
    use crate::ui::ui_manager::UIManager;
    use gtk::Application;
    use std::rc::Rc;

    #[test]
    fn test_playlist_dataprovider() {
        let songs = vec![
            SongInfo::new(String::from("T0"), String::from("A0"), String::from("D0"), None),
            SongInfo::new(String::from("T1"), String::from("A1"), String::from("D1"), None),
            SongInfo::new(String::from("T2"), String::from("A2"), String::from("D2"), None),
        ];
        let mut prov: PlaylistDataProvider = PlaylistDataProvider::new(songs.clone(), Rc::new(UIManager::new(&Application::builder().build())));
        assert_eq!(prov.current(), songs.first());
        prov.next();
        assert_eq!(prov.current(), songs.get(1));
        prov.set_index(2);
        assert_eq!(prov.current(), songs.get(2));
        prov.prev();
        assert_eq!(prov.current(), songs.get(1));
        prov.prev();
        assert_eq!(prov.current(), songs.first());
        prov.prev();
        assert_eq!(prov.current(), songs.first());
    }
}