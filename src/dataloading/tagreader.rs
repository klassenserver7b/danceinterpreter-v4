use std::path::Path;

use id3::{Content, Tag, TagLike};
use id3::frame::Picture;
use id3::Result;

use crate::model::SongInfo;

pub fn read_songinfo_from_file(file: &Path) -> Result<SongInfo> {
    let tag = Tag::read_from_path(file)?;

    let picopt: Option<&Picture> = tag.pictures().next();
    let content: Option<Content> = picopt.map(|pic| Content::Picture(pic.clone()));

    Ok(SongInfo::new(
        tag.title().unwrap_or("").to_string(),
        tag.artist().unwrap_or("").to_string(),
        tag.genre().unwrap_or("").to_string(),
        content,
    ))
}