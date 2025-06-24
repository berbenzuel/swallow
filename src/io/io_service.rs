
use std::path::Path;
use crate::io::io_items::{Directory, File, IoItem, PathItem};



//these are for managing access to filesystem

pub fn read_dir(path: &Path) ->Result<Vec<IoItem>,  std::io::Error> {
    return Ok( std::fs::read_dir(path)?
                   .filter_map(Result::ok)
                   .filter_map(entry_to_io_item)
                   .collect())
}

pub fn entry_to_io_item(entry: std::fs::DirEntry) -> Option<IoItem> {
    match entry.file_type() {
        Ok(file_type) => {
            if file_type.is_dir() {
                Some(IoItem::Directory(Directory::new(entry)))
            }
            else if file_type.is_file() {
                Some(IoItem::File(File::new(entry)))
            }
            else {
                return None;
            }
        }
        Err(_) => {return None}
    }
}