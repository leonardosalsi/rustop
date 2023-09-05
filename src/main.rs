use minitop::setup;

slint::include_modules!();
mod minitop;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    setup();

    ui.run()
}
