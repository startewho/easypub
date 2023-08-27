
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();
}
