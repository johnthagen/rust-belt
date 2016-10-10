//! Main menu.

use find_folder;
use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, text, Transformed};

use color;
use game;
use story;

pub enum MenuSelection {
    Play,
    Story,
    Settings,
    Exit
}

/// Stores Menu state.
pub struct Menu {
    menu_selection: MenuSelection,
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            menu_selection: MenuSelection::Play,
        }
    }

    pub fn run(&mut self, mut window: &mut PistonWindow, game_title: &'static str,
               window_width: u32) {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font_file = assets_folder.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyph_cache = Glyphs::new(font_file, factory).unwrap();
        let menu_align: f64 = ((window_width / 2) - 120) as f64;

        while let Some(event) = window.next() {
            const STARTING_LINE_OFFSET: f64 = 280.0;
            const NEW_LINE_OFFSET: f64 = 40.0;
            const MENU_ITEM_FONT_SIZE: u32 = 32;

            // TODO: Can this be done better with 'if let' ?
            let mut play_color = color::WHITE;
            let mut story_color = color::WHITE;
            let mut settings_color = color::WHITE;
            let mut exit_color = color::WHITE;
            match self.menu_selection {
                MenuSelection::Play => { play_color = color::YELLOW }
                MenuSelection::Story => { story_color = color::YELLOW }
                MenuSelection::Settings => { settings_color = color::YELLOW }
                MenuSelection::Exit => { exit_color = color::YELLOW }
            }

            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               text(color::WHITE,
                                    72,
                                    game_title,
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(menu_align, STARTING_LINE_OFFSET),
                                    graphics);
                               text(play_color,
                                    MENU_ITEM_FONT_SIZE,
                                    "Play",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(menu_align, STARTING_LINE_OFFSET +
                                            1.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(story_color,
                                    MENU_ITEM_FONT_SIZE,
                                    "Story",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(menu_align, STARTING_LINE_OFFSET +
                                            2.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(settings_color,
                                    MENU_ITEM_FONT_SIZE,
                                    "Settings",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(menu_align, STARTING_LINE_OFFSET +
                                            3.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(exit_color,
                                    MENU_ITEM_FONT_SIZE,
                                    "Exit",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(menu_align, STARTING_LINE_OFFSET +
                                            4.0 * NEW_LINE_OFFSET),
                                    graphics);
                           });

            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::W) => {
                        match self.menu_selection {
                            MenuSelection::Play => {}
                            MenuSelection::Story => { self.menu_selection = MenuSelection::Play }
                            MenuSelection::Settings => {
                                self.menu_selection = MenuSelection::Story
                            }
                            MenuSelection::Exit => { self.menu_selection = MenuSelection::Settings }
                        }
                    }
                    Button::Keyboard(Key::S) => {
                        match self.menu_selection {
                            MenuSelection::Play => { self.menu_selection = MenuSelection::Story }
                            MenuSelection::Story => {
                                self.menu_selection = MenuSelection::Settings
                            }
                            MenuSelection::Settings => { self.menu_selection = MenuSelection::Exit }
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
                            MenuSelection::Settings => {

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