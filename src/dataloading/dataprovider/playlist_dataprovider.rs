use crate::model::SongInfo;
use crate::dataloading::dataprovider::VectorForwardBackward;


pub type PlaylistDataProvider<'a> = VectorForwardBackward<'a, SongInfo>;

impl<'a> PlaylistDataProvider<'a> {
    #[allow(dead_code)]
    pub fn current(&self) -> Option<&'a SongInfo>{
        self.vec.get(self.index)
    }

    #[allow(dead_code)]
    pub fn at_index(&mut self, index: usize) -> Option<&'a SongInfo> {
        let current = self.vec.get(index);

        if current.is_some() {
            self.index = index;
        }
        current
    }
}