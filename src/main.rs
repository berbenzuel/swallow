
use std::path::Path;
use crate::io::io_items::{IoItem, PathItem};

mod io;

fn main() {
    println!("Hello, world!");
    
    let user = whoami::username();
    let path = format!("/home/{}/", user);
    let foo = io::io_service::read_dir(Path::new(path.as_str())).unwrap();
    
    
    println!("names:{:?}", 
             foo.iter()
                .map(|item| 
                 match item {
                     IoItem::Directory(dir) => {dir.get_name()},
                     IoItem::File(file) => {file.get_name()},
                 }
                ).collect::<Vec<_>>()
    )
}
