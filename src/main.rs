use std::env;
use std::env::home_dir;
use std::path::Path;
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};
use crate::io::io_manager;

slint::include_modules!();


mod io;
mod ui;

fn main() {
    println!("Hello, world!");


    let user = match whoami::username() {
        Ok(user) => user,
        Err(err) => panic!("{}", err),
    };

    let home_dir = env::home_dir().unwrap();

    let path = home_dir.join("Downloads");
    dbg!(&path.to_str());


    let files = VecModel::default();
    let folders = VecModel::default();

    io_manager::read_dir_as_file_data(Path::new(&path))
        .unwrap()
        .iter().for_each(|item| {
        match item.file_type {
            FileType::File => files.push(item.clone()),
            FileType::Folder => folders.push(item.clone()),
        }
    });

    let win = IoWindow::new().unwrap();
    win.set_files(ModelRc::new(files));
    win.set_folders(ModelRc::new(folders));


    win.on_open_folder(move || {
       println!("open_folder")
    });

    win.run().unwrap();

}

pub fn foo() -> SharedString {
    return SharedString::from("I was called!!");
}

