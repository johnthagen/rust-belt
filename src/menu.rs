//! Main menu.
//! Provides an interface for the user to start the game, change settings, or exit.

use glutin_window::GlutinWindow;
use graphics::{clear, Context, text, Transformed};
use music;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event_loop::Events;
use piston::input::{Button, Input, Key};
use piston::window::Size;

use game;
use game::color::{self, ColoredText};
use settings;
use story;

/// The different soundtrack pieces in the game.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Music {
    /// Menu soundtrack.
    Menu,

    /// Action soundtrack while playing the actual game.
    Action,
}

const MUSIC_FILE_MENU: &str = "./assets/The Last Ranger.mp3";
const MUSIC_FILE_ACTION: &str = "./assets/Into the Field.mp3";

/// The currently selected menu item the user is highlighting.
#[derive(Copy, Clone)]
enum MenuSelection {
    /// Start playing the game.
    Play,

    /// Display the introduction story dialogue.
    Story,

    /// Display the settings screen.
    Settings,

    /// Exit the game.
    Exit,
}

/// Draws the title and menu options to screen.
/// The current menu selection is highlighted based upon user input.
fn draw(context: Context,
        graphics: &mut GlGraphics,
        glyph_cache: &mut GlyphCache,
        menu_align: f64,
        menu_selection: MenuSelection,
        game_title: &'static str) {
    let starting_line_offset = 280.0;

    // Color all menu items the same unless it is currently selected.
    let mut play_color = color::WHITE;
    let mut story_color = color::WHITE;
    let mut settings_color = color::WHITE;
    let mut exit_color = color::WHITE;
    match menu_selection {
        MenuSelection::Play => play_color = color::YELLOW,
        MenuSelection::Story => story_color = color::YELLOW,
        MenuSelection::Settings => settings_color = color::YELLOW,
        MenuSelection::Exit => exit_color = color::YELLOW,
    }

    let menu_lines = [ColoredText {
                          color: play_color,
                          text: "Play",
                      },
                      ColoredText {
                          color: story_color,
                          text: "Story",
                      },
                      ColoredText {
                          color: settings_color,
                          text: "Settings",
                      },
                      ColoredText {
                          color: exit_color,
                          text: "Exit",
                      }];

    clear(color::BLACK, graphics);
    text(color::WHITE,
         72,
         game_title,
         glyph_cache,
         context.transform.trans(menu_align, starting_line_offset),
         graphics);

    for (index, line) in menu_lines.iter().enumerate() {
        let new_line_offset = 40.0;
        text(line.color,
             32,
             line.text,
             glyph_cache,
             context
                 .transform
                 .trans(menu_align,
                        starting_line_offset + ((index as f64 + 1.0) * new_line_offset)),
             graphics);
    }
}

/// Loops the menu screen, taking user input to change the current menu selection.
pub fn run(mut events: &mut Events,
           mut window: &mut GlutinWindow,
           mut opengl: &mut GlGraphics,
           game_title: &'static str,
           window_size: Size) {
    music::start::<Music, _>(|| {
        music::bind_file(Music::Menu, MUSIC_FILE_MENU);
        music::bind_file(Music::Action, MUSIC_FILE_ACTION);
        music::play(&Music::Menu, music::Repeat::Forever);

        // The glyphe cache is mutable because it loads each character on demand (lazily),
        // and thus must be able to be changed over time as new characters are requested.
        let mut glyph_cache = GlyphCache::new("./assets/FiraSans-Regular.ttf").unwrap();

        let mut menu_selection = MenuSelection::Play;
        let mut volume = music::MAX_VOLUME;
        music::set_volume(volume);

        let menu_align = (window_size.width / 2 - 120) as f64;

        while let Some(event) = events.next(window) {
            match event {
                Input::Render(args) => {
                    opengl.draw(args.viewport(), |context, graphics| {
                        draw(context,
                             graphics,
                             &mut glyph_cache,
                             menu_align,
                             menu_selection,
                             game_title)
                    });
                }

                Input::Press(Button::Keyboard(key)) => {
                    match key {
                        Key::W => {
                            match menu_selection {
                                MenuSelection::Play => {}
                                MenuSelection::Story => menu_selection = MenuSelection::Play,
                                MenuSelection::Settings => menu_selection = MenuSelection::Story,
                                MenuSelection::Exit => menu_selection = MenuSelection::Settings,
                            }
                        }
                        Key::S => {
                            match menu_selection {
                                MenuSelection::Play => menu_selection = MenuSelection::Story,
                                MenuSelection::Story => menu_selection = MenuSelection::Settings,
                                MenuSelection::Settings => menu_selection = MenuSelection::Exit,
                                MenuSelection::Exit => {}
                            }
                        }
                        Key::Space => {
                            match menu_selection {
                                MenuSelection::Play => {
                                    music::play(&Music::Action, music::Repeat::Forever);
                                    game::Game::new(window_size).run(&mut events,
                                                                     &mut window,
                                                                     &mut opengl,
                                                                     &mut glyph_cache);
                                    music::play(&Music::Menu, music::Repeat::Forever);
                                }
                                MenuSelection::Story => {
                                    story::run(&mut events,
                                               &mut window,
                                               &mut opengl,
                                               &mut glyph_cache);
                                }
                                MenuSelection::Settings => {
                                    settings::run(&mut events,
                                                  &mut window,
                                                  &mut opengl,
                                                  &mut glyph_cache,
                                                  &mut volume,
                                                  menu_align);
                                }
                                MenuSelection::Exit => break,
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
