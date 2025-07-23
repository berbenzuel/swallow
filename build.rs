use std::path::Path;

fn main() {
    slint_build::compile(Path::new("ui/windows/io_window.slint")).unwrap();
}