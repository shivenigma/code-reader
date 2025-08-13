use eframe::{egui, App, CreationContext, Frame};
use env_logger::Env;
use log::info;
use std::path::PathBuf;

mod app;
mod editor;
mod file_explorer;
mod ui;

use app::CodeReaderApp;

fn main() -> Result<(), eframe::Error> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    info!("Starting Code Reader application");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Code Reader",
        options,
        Box::new(|cc| Box::new(CodeReaderApp::new(cc))),
    )
}
