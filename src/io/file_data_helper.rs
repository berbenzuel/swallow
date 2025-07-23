use std::collections::HashMap;
use std::fs::{DirEntry};
use slint::{slint, SharedString};

use crate::FileType;
use crate::FileData;
impl FileData {

    pub fn new(dir_entry: &DirEntry) -> FileData{
        return FileData::from(dir_entry);
    }

    fn get_file_type(dir_entry: &DirEntry) -> FileType {
        let file_type = dir_entry.metadata().unwrap().file_type();

        if file_type.is_file() {
            return FileType::File;
        }

        else if file_type.is_dir() {
            return FileType::Folder;
        }

        else {
            return FileType::File;
        }
    }
}

impl From<&DirEntry> for FileData {
    fn from(dir_entry: &DirEntry) -> Self {
        Self {
            name: SharedString::from(dir_entry.file_name().to_string_lossy().to_string()),
            path: SharedString::from(dir_entry.path().to_string_lossy().to_string()),
            file_type: FileData::get_file_type(&dir_entry),
        }
    }
}
