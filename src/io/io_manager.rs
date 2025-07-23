const ROOT_DIR_UNIX: &str = "/";


use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::string::ToString;
use crate::FileData;

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

pub fn create_dir(path: &Path, dir_name: String) ->Result<(),  std::io::Error> {
    return std::fs::create_dir(Path::new(path).join(&dir_name));
}

pub fn delete_dir(path: &Path) ->Result<(),  std::io::Error> {
    return std::fs::remove_dir_all(path);
}

pub fn create_file(file_path: &Path, ) {

}
