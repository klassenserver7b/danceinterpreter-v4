use crate::model::SongInfo;
use crate::dataloading::dataprovider::VectorForwardBackward;


pub type PlaylistDataProvider<'a> = VectorForwardBackward<'a, SongInfo>;