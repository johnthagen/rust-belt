extern crate piston_window;

use piston_window::{Button, clear, Key, PressEvent, PistonWindow, polygon, Transformed,
    WindowSettings};

mod color {
    use piston_window::{types};

    pub const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
    pub const CYAN: types::Color = [0.0, 1.0, 1.0, 1.0];
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Rust Belt", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    while let Some(event) = window.next() {
        window.draw_2d(&event,
                       |_context, graphics| {
                           clear(color::BLACK, graphics);
                       });

        if event.press_args().is_some() {
            InnerApp {
                exit_button: Button::Keyboard(Key::X),
                position: Position { x: 0.0, y: 0.0 },
                rotation: 0.0,
            }.run(&mut window);
        }
    }
}

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static [[f64; 2]] = &[
    [0.0, -1.0 * SHIP_HEIGHT / 2.0],
    [SHIP_WIDTH, 0.0],
    [0.0, SHIP_HEIGHT / 2.0]
];

/// Stores application state of inner event loop.
struct InnerApp {
    exit_button: Button,
    position: Position,
    rotation: f64,
}

struct Position {
    x: f64,
    y: f64
}

impl InnerApp {
    fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               polygon(color::CYAN,
                                       SHIP,
                                       context.transform
                                           .trans(self.position.x,
                                                  self.position.y)
                                           .rot_rad(self.rotation)
                                           // Without this trans(), rotation occurs around the
                                           // upper left corner rather than the center.
                                           .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                       graphics);
                           });
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::D) => { self.position.x += 1.0 }
                    Button::Keyboard(Key::A) => { self.position.x -= 1.0 }
                    Button::Keyboard(Key::S) => { self.position.y += 1.0 }
                    Button::Keyboard(Key::W) => { self.position.y -= 1.0 }
                    Button::Keyboard(Key::Q) => { self.rotation -= 0.1 }
                    Button::Keyboard(Key::E) => { self.rotation += 0.1 }
                    _ => {
                        if button == self.exit_button {
                            break;
                        }
                    }
                }
            }
        }
    }
}