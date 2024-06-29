use crate::model::SongInfo;

pub struct PlaylistDataProvider {
    pub songs: Vec<SongInfo>,
    pub index: usize,
}

impl PlaylistDataProvider {
    pub fn new(songs: Vec<SongInfo>) -> Self {
        Self {
            songs,
            index: 0,
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&SongInfo> {
        let next = self.songs.get(self.index + 1);

        if next.is_some() {
            self.index += 1;
        }
        next
    }

    pub fn current(&self) -> Option<&SongInfo> {
        self.songs.get(self.index)
    }

    pub fn previous(&mut self) -> Option<&SongInfo> {
        let prev = self.songs.get(self.index - 1);

        if prev.is_some() {
            self.index -= 1;
        }
        prev
    }

    pub fn at_index(&mut self, index: usize) -> Option<&SongInfo> {
        let current = self.songs.get(index);

        if current.is_some() {
            self.index = index;
        }
        current
    }
}