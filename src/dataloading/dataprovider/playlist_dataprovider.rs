// use std::rc::Rc;
//
// pub struct PlaylistDataProvider
// {
//     pub index: usize,
//     pub vec: Vec<SongInfo>,
//
//     empty_song_info: SongInfo,
//     pub ui_manager: Rc<UIManager>,
// }
//
// impl PlaylistDataProvider {
//     pub fn new(vec: Vec<SongInfo>, ui_manager: Rc<UIManager>) -> Self {
//         Self {
//             vec,
//             index: 0,
//             empty_song_info: SongInfo::new(
//                 0,
//                 String::new(),
//                 String::new(),
//                 String::new(),
//                 None
//             ),
//             ui_manager,
//         }
//     }
//
//     pub fn set_vec(&mut self, vec: Vec<SongInfo>) {
//         self.vec = vec;
//         self.index = 0;
//         self.provide_data();
//     }
//
//     pub fn provide_data(&self) {
//         let current_song = self.vec.get(self.index)
//             .unwrap_or(&self.empty_song_info);
//
//         let next_dance = self.vec.get(self.index + 1)
//             .map(|next_info| next_info.dance())
//             .unwrap_or_default();
//
//         self.ui_manager.set_song_info(current_song, next_dance.as_str());
//         self.ui_manager.set_song_list(&self.vec);
//     }
//
//     pub fn current(&self) -> Option<&SongInfo> {
//         self.vec.get(self.index)
//     }
//
//     pub fn next(&mut self) {
//         if self.vec.get(self.index + 1).is_some() {
//             self.index += 1;
//             self.provide_data();
//         }
//     }
//
//     pub fn set_index(&mut self, n: usize) {
//         if self.vec.get(n).is_some() {
//             self.index = n;
//             self.provide_data();
//         }
//     }
//
//     pub fn prev(&mut self) {
//         if self.index == 0 {
//             return;
//         }
//
//         self.index -= 1;
//         self.provide_data();
//     }
//
//     pub fn get_ui_manager(&self) -> Rc<UIManager> {
//         self.ui_manager.clone()
//     }
// }
