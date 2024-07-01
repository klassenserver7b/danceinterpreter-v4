pub mod playlist_dataprovider;

pub trait ForwardBackwardIterator : Iterator {
    fn prev(&mut self) -> Option<Self::Item>;
}

pub struct VectorForwardBackward<'a, Item> where Item : 'a {
    pub index: usize,
    pub vec: &'a Vec<Item>
}

impl<'a, Item> VectorForwardBackward<'a, Item> {
    fn new(vec: &'a Vec<Item>) -> Self{
        Self{
            vec,
            index: 0
        }
    }
}

impl<'a, Item> Iterator for VectorForwardBackward<'a, Item> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.vec.get(self.index);
        self.index += 1;
        res
    }
}

impl<'a, Item> ForwardBackwardIterator for VectorForwardBackward<'a, Item> {
    fn prev(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            return None;
        }
        
        self.index -= 1;
        self.vec.get(self.index)
    }
}

mod tests {
    use crate::{dataloading::dataprovider::ForwardBackwardIterator, model::SongInfo};

    use super::playlist_dataprovider::PlaylistDataProvider;


    #[test]
    fn test_playlist_dataprovider(){
        let songs = vec![
            SongInfo::new(String::from("T0"), String::from("A0"), String::from("D0"), None),
            SongInfo::new(String::from("T1"), String::from("A1"), String::from("D1"), None),
            SongInfo::new(String::from("T2"), String::from("A2"), String::from("D2"), None)
        ];
        let mut prov = PlaylistDataProvider::new(&songs);
        assert_eq!(prov.next(), songs.get(0));
        println!("{:?}", prov.next());
        assert_eq!(prov.prev(), songs.get(1));
        assert_eq!(prov.current(), songs.get(1));
        assert_eq!(prov.at_index(2), songs.get(2));
    }
}