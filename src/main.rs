//! Rust belt is a 2D video game inspired by Asteroids.
//! It runs atop the Piston game engine for graphics and SDL2 for sound.

extern crate music;
extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings};
use opengl_graphics::GlGraphics;

mod game;
mod menu;
mod settings;
mod story;

/// Creates a new window and runs the game starts the main menu.
fn main() {
    let game_title = "Rust Belt";
    let game_window_size = Size {
        width: 1024,
        height: 768,
    };

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new(game_title,
                            [game_window_size.width, game_window_size.height])
                .opengl(opengl)
                .samples(4)
                .exit_on_esc(true)
                .build()
                .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut gl = GlGraphics::new(opengl);

    menu::run(&mut window, &mut gl, game_title, game_window_size);
}
