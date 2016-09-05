extern crate piston_window;

use piston_window::{AdvancedWindow, Button, clear, Key, PressEvent, PistonWindow,
    rectangle, Transformed, WindowSettings};

fn main() {
    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    while let Some(event) = window.next() {
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const DARK_GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

        window.draw_2d(&event,
                       |context, graphics| {
                           clear(WHITE, graphics);
                           rectangle(DARK_GREEN,
                                     [50.0, 50.0, 100.0, 100.0],
                                     context.transform,
                                     graphics);
                       });

        if event.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
                position: Position { x: 0.0, y: 0.0 },
                rotation: 0.0,
            }.run(&mut window);
            window.set_title(title.into());
        }
    }
}

const SHIP_SIZE: f64 = 20.0;

/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
    pub position: Position,
    pub rotation: f64,
}

pub struct Position {
    pub x: f64,
    pub y: f64
}

impl InnerApp {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(event) = window.next() {
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];

            window.draw_2d(&event,
                           |context, graphics| {
                               clear(BLACK, graphics);
                               rectangle(CYAN,
                                         rectangle::square(0.0, 0.0, SHIP_SIZE),
                                         context.transform
                                             .trans(self.position.x,
                                                    self.position.y)
                                             .rot_rad(self.rotation)
                                             // Without this trans(), rotation occurs around the
                                             // upper left corner rather than the center.
                                             .trans(-1.0 * SHIP_SIZE / 2.0,
                                                    -1.0 * SHIP_SIZE / 2.0),
                                         graphics);
                           });
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::D) => { self.position.x += 1.0 }
                    Button::Keyboard(Key::A) => { self.position.x -= 1.0 }
                    Button::Keyboard(Key::S) => { self.position.y += 1.0 }
                    Button::Keyboard(Key::W) => { self.position.y -= 1.0 }
                    Button::Keyboard(Key::Q) => { self.rotation += 0.1 }
                    Button::Keyboard(Key::E) => { self.rotation -= 0.1 }
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