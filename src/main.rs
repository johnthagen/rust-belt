extern crate piston_window;
extern crate find_folder;
extern crate music;

mod color;
mod game;
mod menu;
mod settings;
mod story;

use piston_window::{PistonWindow, WindowSettings};

fn main() {
    const GAME_TITLE: &'static str = "Rust Belt";
    const GAME_WINDOW_SIZE: game::Size = game::Size { width: 1024.0, height: 768.0 };

    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_SIZE.width as u32,
                                                        GAME_WINDOW_SIZE.height as u32])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    menu::run(&mut window, GAME_TITLE, GAME_WINDOW_SIZE);
}