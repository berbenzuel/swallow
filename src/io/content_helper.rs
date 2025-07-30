use std::path::PathBuf;
use std::rc::Rc;
use slint::{ModelRc, SharedString, VecModel};
use crate::{FileData, Content};

impl Content {
    pub fn new(files: VecModel<FileData>, folders: VecModel<FileData>, path: PathBuf) -> Self {
        let files_rc = Rc::new(VecModel::from(files));
        let folders_rc = Rc::new(VecModel::from(folders));

        Self {
            files: ModelRc::from(files_rc),
            folders: ModelRc::from(folders_rc),
            path: SharedString::from(path.to_str().unwrap()),
        }
    }
}