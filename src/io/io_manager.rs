const ROOT_DIR_UNIX: &str = "/";


use std::fs::{DirEntry};
use std::{env, path};
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::task::Context;
use slint::VecModel;
use crate::{FileData, Content, FileType};

pub fn read_dir(path: &Path) ->Result<Vec<DirEntry>,  std::io::Error> {
    return Ok(std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>());
}

pub fn read_dir_as_file_data(path: &Path) -> Result<Vec<FileData>, std::io::Error> {
    return Ok(read_dir(path)?.iter()
        .filter_map(|dir_entry| {
            Some(FileData::from(dir_entry))
        }).collect());
}

pub fn get_content(path: &Path) -> Result<Content, std::io::Error> {
    let files = VecModel::default();
    let folders = VecModel::default();
    std::fs::read_dir(path)?
        .for_each( |dir| {
            if let Ok(dir_entry) = dir {
                let file_data = FileData::from(&dir_entry);
                match file_data.file_type {
                    FileType::File => {files.push(file_data)},
                    FileType::Folder => {folders.push(file_data)},
                }
            }
        });
    return Ok(Content::new(files, folders, path.to_path_buf()), );
}


pub fn create_dir(path: &Path, dir_name: String) ->Result<(),  std::io::Error> {
    return std::fs::create_dir(Path::new(path).join(&dir_name));
}

pub fn delete_dir(path: &Path) ->Result<(),  std::io::Error> {
    return std::fs::remove_dir_all(path);
}

pub fn create_file(file_path: &Path, ) {

}

pub fn default_dir() -> Result<path::PathBuf, std::io::Error> {
    return match env::home_dir() {
        Some(path) => Ok(path::PathBuf::from(path)),
        None => panic!("Impossible to get your home dir!"),
    }
}
