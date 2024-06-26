use std::path::{Path, PathBuf};
use std::io::Result;
use url::Url;

pub fn load_m3u_from_path(path: &Path) -> Result<Vec<PathBuf>> {
    let m3u_file = std::fs::read_to_string(path)?;

    let entries = m3u_file.split('\n')
        .filter(|x| !x.starts_with('#'));

    let absolute_paths = entries
        .map(|entry| parse_file_uri(entry).unwrap_or(parse_relative_uri(entry)))
        .map(|entry| path.parent().unwrap().join(entry));

    let accessible_files = absolute_paths
        .filter(|path| path.exists());

    Ok(accessible_files.collect::<Vec<PathBuf>>())
}

fn parse_file_uri(uri: &str) -> Option<PathBuf> {
    let uri = Url::parse(uri).ok()?;
    uri.to_file_path().ok()
}

fn parse_relative_uri(uri: &str) -> PathBuf {
    PathBuf::from(url_decode(uri))
}

fn url_decode(url: &str) -> String {
    let mut decoded = String::from("");
    let mut iter = url.chars();

    while let Some(c) = iter.next() {
        decoded.push(
            if c == '%' {
                let byte = u8::from_str_radix(format!("{}{}", iter.next().unwrap(), iter.next().unwrap()).as_str(), 16).unwrap();
                byte as char
            } else {
                c
            }
        );
    }
    decoded
}

#[cfg(test)]
#[macro_use]
mod tests {
    use std::path::Path;
    use crate::test_file;
    use crate::dataloading::m3uloader::load_m3u_from_path;

    #[test]
    fn it_works() {
        let result = load_m3u_from_path(Path::new(test_file!("m3u_validation_test.m3u")));
        assert!(result.is_ok());
        #[cfg(not(windows))]
        assert_eq!(result.unwrap().len(), 4);
        #[cfg(windows)]
        assert_eq!(result.unwrap().len(), 2);
    }
}