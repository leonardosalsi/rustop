use minitop::setup;
mod minitop;
slint::include_modules!();


fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    setup();

    ui.run()
}
