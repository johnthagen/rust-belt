extern crate piston_window;
extern crate opengl_graphics;
extern crate music;

mod color;
mod game;
mod actor;
mod enemy;
mod player;
mod menu;
mod settings;
mod story;

use piston_window::{OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

fn main() {
    const GAME_TITLE: &'static str = "Rust Belt";
    const GAME_WINDOW_SIZE: game::Size = game::Size {
        width: 1024.0,
        height: 768.0,
    };

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_SIZE.width as u32,
                                                        GAME_WINDOW_SIZE.height as u32])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut gl = GlGraphics::new(opengl);

    menu::run(&mut window, &mut gl, GAME_TITLE, &GAME_WINDOW_SIZE);
}
