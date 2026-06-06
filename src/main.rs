use fltk::{app, button::Button, prelude::*, window::Window, frame::Frame};
use fltk::enums::Color;
use log::info;

mod modules;
mod utils;

use modules::temp_cleaner::TempCleaner;
use modules::prefetch_cleaner::PrefetchCleaner;
use modules::ram_booster::RamBooster;

fn main() {
    env_logger::init();
    info!("Advanced Cleanup Utility 1.4 - Rust Edition Starting");

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(560, 950)
        .with_label("Advanced System Cleanup Utility 1.4 - Rust Edition");
    
    wind.set_color(Color::from_rgb(10, 10, 10));

    let mut header = Frame::default()
        .with_size(500, 60);
    header.set_label("ADVANCED CLEANUP UTILITY 1.4");
    header.set_color(Color::from_rgb(10, 10, 10));
    header.set_label_color(Color::from_rgb(0, 255, 127));

    let mut status = Frame::default()
        .with_size(500, 25);
    status.set_label("System Ready");
    status.set_color(Color::from_rgb(10, 10, 10));
    status.set_label_color(Color::from_rgb(100, 100, 100));

    let mut btn_exit = Button::default()
        .with_size(500, 55);
    btn_exit.set_label("Exit Utility");
    btn_exit.set_color(Color::from_rgb(220, 38, 38));

    wind.end();
    wind.show();

    btn_exit.set_callback(|_| {
        std::process::exit(0);
    });

    app.run().unwrap();
}
