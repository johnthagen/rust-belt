//! Main menu.

use music;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Context, Event, Input, Key, PistonWindow, text, Transformed,
    types};

use color;
use game;
use settings;
use story;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    Menu,
    Action,
}

#[derive(Copy, Clone)]
enum MenuSelection {
    Play,
    Story,
    Settings,
    Exit
}

fn render(context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache,
          menu_align: f64, menu_selection: MenuSelection, game_title: &'static str) {
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

    clear(color::BLACK, graphics);
    text(color::WHITE,
         72,
         game_title,
         glyph_cache,
         context.transform
             .trans(menu_align, STARTING_LINE_OFFSET),
         graphics);
    text(play_color,
         MENU_ITEM_FONT_SIZE,
         "Play",
         glyph_cache,
         context.transform
             .trans(menu_align, STARTING_LINE_OFFSET +
                 1.0 * NEW_LINE_OFFSET),
         graphics);
    text(story_color,
         MENU_ITEM_FONT_SIZE,
         "Story",
         glyph_cache,
         context.transform
             .trans(menu_align, STARTING_LINE_OFFSET +
                 2.0 * NEW_LINE_OFFSET),
         graphics);
    text(settings_color,
         MENU_ITEM_FONT_SIZE,
         "Settings",
         glyph_cache,
         context.transform
             .trans(menu_align, STARTING_LINE_OFFSET +
                 3.0 * NEW_LINE_OFFSET),
         graphics);
    text(exit_color,
         MENU_ITEM_FONT_SIZE,
         "Exit",
         glyph_cache,
         context.transform
             .trans(menu_align, STARTING_LINE_OFFSET +
                 4.0 * NEW_LINE_OFFSET),
         graphics);
}

pub fn run(mut window: &mut PistonWindow, mut opengl: &mut GlGraphics, game_title: &'static str,
           window_size: &game::Size) {
    music::start::<Music, _>(|| {
        music::bind_file(Music::Menu, "./assets/The Last Ranger.mp3");
        music::bind_file(Music::Action, "./assets/Into the Field.mp3");
        music::play(&Music::Menu, music::Repeat::Forever);

        let mut glyph_cache = GlyphCache::new("./assets/FiraSans-Regular.ttf").unwrap();

        let mut menu_selection = MenuSelection::Play;
        let mut volume = music::MAX_VOLUME;

        let menu_align = (window_size.width / 2.0) - 120.0;

        while let Some(event) = window.next() {
            match event {
                Event::Render(args) => {
                    opengl.draw(args.viewport(),
                                |context, graphics| {
                                    render(context, graphics, &mut glyph_cache, menu_align,
                                           menu_selection, game_title)
                                });
                }

                Event::Input(Input::Press(Button::Keyboard(key))) => {
                    match key {
                        Key::W => {
                            match menu_selection {
                                MenuSelection::Play => {}
                                MenuSelection::Story => { menu_selection = MenuSelection::Play }
                                MenuSelection::Settings => {
                                    menu_selection = MenuSelection::Story
                                }
                                MenuSelection::Exit => { menu_selection = MenuSelection::Settings }
                            }
                        }
                        Key::S => {
                            match menu_selection {
                                MenuSelection::Play => { menu_selection = MenuSelection::Story }
                                MenuSelection::Story => {
                                    menu_selection = MenuSelection::Settings
                                }
                                MenuSelection::Settings => { menu_selection = MenuSelection::Exit }
                                MenuSelection::Exit => {}
                            }
                        }
                        Key::Space => {
                            match menu_selection {
                                MenuSelection::Play => {
                                    music::play(&Music::Action, music::Repeat::Forever);
                                    game::Game::new().run(&mut window, &mut opengl, window_size);
                                    music::play(&Music::Menu, music::Repeat::Forever);
                                }
                                MenuSelection::Story => {
                                    story::run(&mut window, &mut opengl, &mut glyph_cache);
                                }
                                MenuSelection::Settings => {
                                    settings::run(&mut window, &mut opengl, &mut glyph_cache,
                                                  &mut volume, menu_align);
                                }
                                MenuSelection::Exit => { break }
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    });
}
