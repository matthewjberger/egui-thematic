#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use egui_thematic_demo::DemoApp;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("egui-thematic Demo"),
        ..Default::default()
    };

    eframe::run_native(
        "egui-thematic Demo",
        native_options,
        Box::new(|_cc| Ok(Box::<DemoApp>::default())),
    )
}
