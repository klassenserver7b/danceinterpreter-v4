use std::path::Path;

use id3::{Tag, TagLike};
use id3::frame::PictureType;
use id3::Result;

use crate::model::SongInfo;

pub fn read_song_info_from_file(file: &Path) -> Result<SongInfo> {
    let tag = Tag::read_from_path(file)?;

    let album_art = tag.pictures()
        // use front cover if available
        .filter(|pic| pic.picture_type == PictureType::CoverFront).next()
        // otherwise try to use any picture
        .or(tag.pictures().next())
        .map(|pic| pic.clone());

    Ok(SongInfo::new(
        tag.title().unwrap_or("").to_string(),
        tag.artist().unwrap_or("").to_string(),
        tag.genre().unwrap_or("").to_string(),
        album_art,
    ))
}