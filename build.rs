use std::path::Path;

fn main() {
    slint_build::compile(Path::new("ui/app_main.slint")).unwrap();
}