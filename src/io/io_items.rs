use std::cell::RefMut;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

pub enum IoItem {
    File(File),
    Directory(Directory),
}



pub trait PathItem
{
    fn new(entry: DirEntry) -> Self;
    fn get_path(&self) -> PathBuf;
    fn get_name(&self) -> String;
}


//file item
pub struct File {
    pub entry: DirEntry,
}
impl PathItem for File {
    fn new(entry: DirEntry) -> File {
        return File{
            entry: entry,
        }
    }
    fn get_path(&self) -> PathBuf {
        return self.entry.path();
    }
    fn get_name(&self) -> String {
        return String::from(self.entry.file_name().to_str().unwrap_or_default());
    }
}


//directory item
pub struct Directory {
    pub entry: DirEntry,
}
impl PathItem for Directory {
    fn new(entry: DirEntry) -> Directory {
        return Directory{
            entry
        }
    }
    fn get_path(&self) -> PathBuf {
        return self.entry.path();
    }
    fn get_name(&self) -> String {
        return String::from(self.entry.file_name().to_str().unwrap_or_default());
    }



}
