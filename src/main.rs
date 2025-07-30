use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::env::home_dir;
use std::path::Path;
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};
use log::log;
use slint::private_unstable_api::debug;
use slint::private_unstable_api::re_exports::SharedVectorModel;
use crate::io::io_manager;

slint::include_modules!();


mod io;
mod ui;

//Box dyn Error -- it can return any error which implements Error trait:) (gonna kill myself)
fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world!");

    let app = AppMain::new()?;

    let model: Rc<VecModel<Content>> =(Rc::new(VecModel::default()));

    _ = ui::handle_callbacks(&app, &model);

    _ = ui::spawn_new_window(&model);
    _ = ui::spawn_new_window(&model);

    let model = ModelRc::from(model);

    app.set_model(model);

    app.run().unwrap();
    return Ok(());
}

pub fn foo() -> SharedString {
    return SharedString::from("I was called!!");
}

