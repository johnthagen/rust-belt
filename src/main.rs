extern crate piston_window;
extern crate find_folder;
extern crate music;

mod color;
mod game;
mod story;

use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, text, Transformed,
    WindowSettings};

const GAME_TITLE: &'static str = "Rust Belt";
const GAME_WINDOW_WIDTH: u32 = 1024;
const GAME_WINDOW_HEIGHT: u32 = 768;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Menu,
    Action,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    music::start::<Music, _>(|| {
        music::bind_file(Music::Menu, "./assets/The Last Ranger.mp3");
        music::bind_file(Music::Action, "./assets/Into the Field.mp3");
        music::play(&Music::Menu, music::Repeat::Forever);

        Menu {
            menu_selection: MenuSelection::Play,
        }.run(&mut window);
    });
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
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font_file = assets_folder.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyph_cache = Glyphs::new(font_file, factory).unwrap();

        while let Some(event) = window.next() {
            const MENU_ALIGN: f64 = ((GAME_WINDOW_WIDTH / 2) - 120) as f64;
            const STARTING_LINE_OFFSET: f64 = 280.0;
            const NEW_LINE_OFFSET: f64 = 40.0;

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
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET),
                                    graphics);
                               text(play_color,
                                    32,
                                    "Play",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            1.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(story_color,
                                    32,
                                    "Story",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            2.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(exit_color,
                                    32,
                                    "Exit",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            3.0 * NEW_LINE_OFFSET),
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
                                game::Game {
                                    position: game::Position { x: 0.0, y: 0.0 },
                                    rotation: 0.0,
                                }.run(&mut window);
                            }
                            MenuSelection::Story => {
                                story::run(&mut window, font_file);
                            }
                            MenuSelection::Exit => { break }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}