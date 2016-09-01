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
                       |context, graphics2d| {
                           clear([1.0, 1.0, 1.0, 1.0], //color
                                 graphics2d);
                           rectangle([1.0, 0.0, 1.0, 1.0], // color
                                     [0.0, 0.0, 100.0, 100.0], // border
                                     context.transform, graphics2d);
                       });
    }
}