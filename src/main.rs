extern crate piston_window;

use piston_window::{AdvancedWindow, Button, clear, ellipse, Key, PressEvent, PistonWindow,
    rectangle, WindowSettings};

fn main() {
    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    while let Some(event) = window.next() {
        window.draw_2d(&event,
                       |context, graphics| {
                            clear([0.5, 1.0, 0.5, 1.0], graphics);
                            rectangle([1.0, 0.0, 0.0, 1.0],
                                      [50.0, 50.0, 100.0, 100.0],
                                      context.transform,
                                      graphics);
        });

        if event.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
                position: Position{ x: 50.0, y: 50.0 },
            }.run(&mut window);
            window.set_title(title.into());
        }
    }
}

/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
    pub position: Position,
}

pub struct Position {
    pub x: f64,
    pub y: f64
}

impl InnerApp {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(event) = window.next() {
            window.draw_2d(&event,
                           |context, graphics| {
                                clear([0.5, 0.5, 1.0, 1.0], graphics);
                                ellipse([1.0, 0.0, 0.0, 1.0],
                                        [self.position.x, self.position.y, 20.0, 20.0],
                                        context.transform,
                                        graphics);
            });
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::D) => { self.position.x += 1.0 }
                    Button::Keyboard(Key::A) => { self.position.x -= 1.0 }
                    Button::Keyboard(Key::S) => { self.position.y += 1.0 }
                    Button::Keyboard(Key::W) => { self.position.y -= 1.0 }
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