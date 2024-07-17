use crate::dataloading::dataprovider::VectorForwardBackward;
use crate::model::SongInfo;

pub type PlaylistDataProvider<'a> = VectorForwardBackward<'a, SongInfo>;