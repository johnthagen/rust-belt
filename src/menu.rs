//! Main menu.

use find_folder;
use music;
use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, text, Transformed, types};

use color;
use game;
use settings;
use story;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Menu,
    Action,
}

enum MenuSelection {
    Play,
    Story,
    Settings,
    Exit
}

pub fn run(mut window: &mut PistonWindow, game_title: &'static str, window_width: u32) {
    music::start::<Music, _>(|| {
        music::bind_file(Music::Menu, "./assets/The Last Ranger.mp3");
        music::bind_file(Music::Action, "./assets/Into the Field.mp3");
        music::play(&Music::Menu, music::Repeat::Forever);

        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font_file = assets_folder.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyph_cache = Glyphs::new(font_file, factory).unwrap();
        let menu_align: f64 = ((window_width / 2) - 120) as f64;

        let mut menu_selection = MenuSelection::Play;
        let mut volume = music::MAX_VOLUME;

        while let Some(event) = window.next() {
            const STARTING_LINE_OFFSET: f64 = 280.0;
            const NEW_LINE_OFFSET: f64 = 40.0;
            const MENU_ITEM_FONT_SIZE: types::FontSize = 32;

            // TODO: Can this be done better with 'if let' ?
            let mut play_color = color::WHITE;
            let mut story_color = color::WHITE;
            let mut settings_color = color::WHITE;
            let mut exit_color = color::WHITE;
            match menu_selection {
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
                        match menu_selection {
                            MenuSelection::Play => {}
                            MenuSelection::Story => { menu_selection = MenuSelection::Play }
                            MenuSelection::Settings => {
                                menu_selection = MenuSelection::Story
                            }
                            MenuSelection::Exit => { menu_selection = MenuSelection::Settings }
                        }
                    }
                    Button::Keyboard(Key::S) => {
                        match menu_selection {
                            MenuSelection::Play => { menu_selection = MenuSelection::Story }
                            MenuSelection::Story => {
                                menu_selection = MenuSelection::Settings
                            }
                            MenuSelection::Settings => { menu_selection = MenuSelection::Exit }
                            MenuSelection::Exit => {}
                        }
                    }
                    Button::Keyboard(Key::Space) => {
                        match menu_selection {
                            MenuSelection::Play => {
                                music::play(&Music::Action, music::Repeat::Forever);
                                game::Game::new().run(&mut window);
                                music::play(&Music::Menu, music::Repeat::Forever);
                            }
                            MenuSelection::Story => {
                                story::run(&mut window, font_file);
                            }
                            MenuSelection::Settings => {
                                settings::run(&mut window, font_file, &mut volume, menu_align);
                            }
                            MenuSelection::Exit => { break }
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}
