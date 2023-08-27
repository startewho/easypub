
slint::include_modules!();
mod epub;
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_epub(move |s|{
        epub::gen(s)
    });

    ui.run()
}
