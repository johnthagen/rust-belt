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

fn main() {
    const GAME_TITLE: &'static str = "Rust Belt";
    const GAME_WINDOW_SIZE: Size = Size {
        width: 1024,
        height: 768,
    };

    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_SIZE.width,
                                                        GAME_WINDOW_SIZE.height])
        .opengl(opengl)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut gl = GlGraphics::new(opengl);

    menu::run(&mut window, &mut gl, GAME_TITLE, GAME_WINDOW_SIZE);
}
