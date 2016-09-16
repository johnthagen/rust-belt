extern crate piston_window;
extern crate find_folder;

use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, polygon, text,
    Transformed, WindowSettings};

mod color {
    use piston_window::{types};

    pub const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
    pub const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];
    pub const CYAN: types::Color = [0.0, 1.0, 1.0, 1.0];
    pub const YELLOW: types::Color = [1.0, 1.0, 0.0, 1.0];
}

const GAME_TITLE: &'static str = "Rust Belt";
const GAME_WINDOW_WIDTH: u32 = 640;
const GAME_WINDOW_HEIGHT: u32 = 480;


fn main() {
    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    Menu {
        menu_selection: MenuSelection::Play,
    }.run(&mut window);
}

enum MenuSelection {
    Play,
    Story,
    Exit
}

/// Stores Menu state.
struct Menu {
    menu_selection: MenuSelection,
}

impl Menu {
    fn run(&mut self, mut window: &mut PistonWindow) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        println!("{:?}", assets);
        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyphs = Glyphs::new(font, factory).unwrap();

        while let Some(event) = window.next() {
            const MENU_ALIGN: f64 = ((GAME_WINDOW_WIDTH / 2) - 120) as f64;

            // TODO: Can this be done better with 'if let' ?
            let mut play_color = color::WHITE;
            let mut story_color = color::WHITE;
            let mut exit_color = color::WHITE;
            match self.menu_selection {
                MenuSelection::Play => { play_color = color::YELLOW }
                MenuSelection::Story => { story_color = color::YELLOW }
                MenuSelection::Exit => { exit_color = color::YELLOW }
            }

            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               text(color::WHITE,
                                    72,
                                    GAME_TITLE,
                                    &mut glyphs,
                                    context.transform
                                        .trans(MENU_ALIGN, 80.0),
                                    graphics);
                               text(play_color,
                                    32,
                                    "Play",
                                    &mut glyphs,
                                    context.transform
                                        .trans(MENU_ALIGN, 120.0),
                                    graphics);
                               text(story_color,
                                    32,
                                    "Story",
                                    &mut glyphs,
                                    context.transform
                                        .trans(MENU_ALIGN, 160.0),
                                    graphics);
                               text(exit_color,
                                    32,
                                    "Exit",
                                    &mut glyphs,
                                    context.transform
                                        .trans(MENU_ALIGN, 200.0),
                                    graphics);
                           });

            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::W) => {
                        match self.menu_selection {
                            MenuSelection::Play => {}
                            MenuSelection::Story => { self.menu_selection = MenuSelection::Play }
                            MenuSelection::Exit => { self.menu_selection = MenuSelection::Story }
                        }
                    }
                    Button::Keyboard(Key::S) => {
                        match self.menu_selection {
                            MenuSelection::Play => { self.menu_selection = MenuSelection::Story }
                            MenuSelection::Story => { self.menu_selection = MenuSelection::Exit }
                            MenuSelection::Exit => {}
                        }
                    }
                    Button::Keyboard(Key::Space) => {
                        match self.menu_selection {
                            MenuSelection::Play => {
                                GamePlay {
                                    position: Position { x: 0.0, y: 0.0 },
                                    rotation: 0.0,
                                }.run(&mut window);
                            }
                            MenuSelection::Exit => { break }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
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

struct Position {
    x: f64,
    y: f64
}

/// Stores Game state.
struct GamePlay {
    position: Position,
    rotation: f64,
}

impl GamePlay {
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
                    Button::Keyboard(Key::X) => { break }
                    _ => {}
                }
            }
        }
    }
}