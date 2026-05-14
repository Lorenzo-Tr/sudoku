#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod board;
mod puzzle;

fn main() -> iced::Result {
    app::run()
}
