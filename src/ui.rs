use crate::IoWindowAdapter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::{Rc};
use slint::private_unstable_api::re_exports::SharedVectorModel;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};
use crate::{io, AppMain, Content};
use std::thread;
use log::debug;
use slint::private_unstable_api::debug;

pub fn handle_callbacks(ui_handler: &AppMain, model_handler: &Rc<slint::VecModel<Content>>) -> Result<(), std::io::Error> {

    ui_handler.global::<IoWindowAdapter>().on_open_folder({
        let app_weak = ui_handler.as_weak();
        let model = model_handler.clone();

        move |path: SharedString, index: i32| {
            let model = model.clone(); //making a shared reference -> overf*ck scoping
            debug(path.clone());
            model.set_row_data(
                index as usize,
                io::io_manager::get_content(Path::new(&path.to_string())).unwrap());
            app_weak.unwrap().set_model(model.into());
        }}
    );

    ui_handler.global()::<IoWindowAdapter>().on_open_parent_folder({

    });

    return Ok(());
}

pub fn spawn_new_window(model_handler: &Rc<VecModel<Content>>) -> Result<(), std::io::Error> {
    let def_dir = io::io_manager::default_dir()?;
    let content = io::io_manager::get_content(def_dir.as_path())?;
    model_handler.push(content);
    Ok(())
}