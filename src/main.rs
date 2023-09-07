mod specs {
    pub mod processes;
    pub mod io;
    pub mod network;
    pub mod cpu;
    pub mod info;
}
pub mod ui {
    slint::include_modules!();
}
mod controller;
use controller::start_measurements;
use slint::*;
use ui::*;
use std::env;


fn main() -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    let window_weak: Weak<MainWindow> = window.as_weak();
    start_measurements(window_weak);
    window.run()
}
