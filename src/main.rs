extern crate graphics;
extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use graphics::{clear, rectangle};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Belt", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event,
                       |c, g| {
                           clear([1.0; 4], g);
                           rectangle([1.0, 0.0, 0.0, 1.0], // red
                                     [0.0, 0.0, 100.0, 100.0],
                                     c.transform, g);
                       });
    }
}