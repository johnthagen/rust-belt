//! Modify game settings.

use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{
    clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent, Transformed,
};

use crate::game::color;
use crate::menu::{Sound, Volume};

/// The currently selected menu item the user is highlighting.
#[derive(Copy, Clone)]
enum MenuSelection {
    MusicVolume,
    SoundVolume,
}

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    glyph_cache: &mut GlyphCache<'_>,
    menu_selection: MenuSelection,
    volume: Volume,
    left_alignment: f64,
) {
    // Color all menu items the same unless it is currently selected.
    let mut music_color = color::WHITE;
    let mut sound_color = color::WHITE;
    match menu_selection {
        MenuSelection::MusicVolume => music_color = color::YELLOW,
        MenuSelection::SoundVolume => sound_color = color::YELLOW,
    }

    let menu_font = 32;
    let starting_line_offset = 280.0;
    let value_left_alignment = left_alignment + 300.0;
    let new_line_offset = 40.0;

    clear(color::BLACK, graphics);
    text(
        music_color,
        menu_font,
        "Music Volume",
        glyph_cache,
        context
            .transform
            .trans(left_alignment, starting_line_offset),
        graphics,
    )
    .unwrap();
    text(
        music_color,
        menu_font,
        &format!("{}%", (volume.music * 100.0) as i32),
        glyph_cache,
        context
            .transform
            .trans(value_left_alignment, starting_line_offset),
        graphics,
    )
    .unwrap();
    text(
        sound_color,
        menu_font,
        "Sound Volume",
        glyph_cache,
        context
            .transform
            .trans(left_alignment, starting_line_offset + new_line_offset),
        graphics,
    )
    .unwrap();
    text(
        sound_color,
        menu_font,
        &format!("{}%", (volume.sound * 100.0) as i32),
        glyph_cache,
        context
            .transform
            .trans(value_left_alignment, starting_line_offset + new_line_offset),
        graphics,
    )
    .unwrap();
}

/// Loop providing game setting options to change to the user until they exit the screen.
pub fn run(
    window: &mut PistonWindow,
    opengl: &mut GlGraphics,
    glyph_cache: &mut GlyphCache<'_>,
    volume: &mut Volume,
    left_alignment: f64,
) {
    let mut menu_selection = MenuSelection::MusicVolume;

    for event in window.by_ref() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                draw(
                    context,
                    graphics,
                    glyph_cache,
                    menu_selection,
                    *volume,
                    left_alignment,
                )
            });
        }

        // TODO: Known precision problem related to stepping f64 instead of integers.
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let volume_step: f64 = 0.1;

            match key {
                Key::W => match menu_selection {
                    MenuSelection::MusicVolume => {}
                    MenuSelection::SoundVolume => menu_selection = MenuSelection::MusicVolume,
                },
                Key::S => match menu_selection {
                    MenuSelection::MusicVolume => menu_selection = MenuSelection::SoundVolume,
                    MenuSelection::SoundVolume => {}
                },
                Key::D => {
                    music::play_sound(&Sound::MenuSelection, music::Repeat::Times(0), volume.sound);
                    match menu_selection {
                        MenuSelection::MusicVolume => volume.music += volume_step,
                        MenuSelection::SoundVolume => volume.sound += volume_step,
                    }
                }
                Key::A => {
                    music::play_sound(&Sound::MenuSelection, music::Repeat::Times(0), volume.sound);
                    match menu_selection {
                        MenuSelection::MusicVolume => volume.music -= volume_step,
                        MenuSelection::SoundVolume => volume.sound -= volume_step,
                    }
                }
                Key::Space => {
                    music::play_sound(&Sound::MenuBack, music::Repeat::Times(0), volume.sound);
                    break;
                }
                _ => {}
            }

            volume.music = volume.music.max(music::MIN_VOLUME).min(music::MAX_VOLUME);
            volume.sound = volume.sound.max(music::MIN_VOLUME).min(music::MAX_VOLUME);
            music::set_volume(volume.music);
        }
    }
}
