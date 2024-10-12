use id3::frame::PictureType;
use id3::Result;
use id3::{Tag, TagLike};
use std::path::Path;

// pub fn read_song_info_from_filepath(file: &Path) -> Result<SongInfo> {
//     let tag = Tag::read_from_path(file)?;
// 
//     let album_art = tag.pictures()
//         // use front cover if available
//         .find(|pic| pic.picture_type == PictureType::CoverFront)
//         // otherwise try to use any picture
//         .or(tag.pictures().next())
//         .cloned();
// 
//     Ok(SongInfo::new(
//         tag.track().unwrap_or(0),
//         tag.title().unwrap_or("").to_string(),
//         tag.artist().unwrap_or("").to_string(),
//         tag.genre().unwrap_or("").to_string(),
//         album_art,
//     )
//     )
// }
// 
// pub fn read_song_info_from_file(file: &File) -> Result<SongInfo> {
//     read_song_info_from_filepath(&file.path().unwrap())
// }
// 
// pub fn read_song_info_from_filepaths(files: &[&Path]) -> Vec<Result<SongInfo>> {
//     files.iter().map(|file| read_song_info_from_filepath(file)).collect()
// }
// 
// pub fn read_song_info_from_files(file_list: &FileList) -> Vec<Result<SongInfo>> {
//     file_list.files().iter().map(read_song_info_from_file).collect()
// }