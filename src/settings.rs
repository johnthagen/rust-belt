//! Modify game settings.

use music;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, Context, clear, Input, Key, PistonWindow, text, Transformed};

use game::color;
use menu::Sound;

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    glyph_cache: &mut GlyphCache,
    volume: f64,
    left_alignment: f64,
) {
    let starting_line_offset = 280.0;
    let value_left_alignment = left_alignment + 300.0;

    clear(color::BLACK, graphics);
    text(
        color::WHITE,
        32,
        "Volume",
        glyph_cache,
        context
            .transform
            .trans(left_alignment, starting_line_offset),
        graphics,
    );
    text(
        color::WHITE,
        32,
        &format!("{}%", (volume * 100.0) as i32),
        glyph_cache,
        context
            .transform
            .trans(value_left_alignment, starting_line_offset),
        graphics,
    );
}

/// Loop providing game setting options to change to the user until they exit the screen.
pub fn run(
    window: &mut PistonWindow,
    opengl: &mut GlGraphics,
    glyph_cache: &mut GlyphCache,
    volume: &mut f64,
    left_alignment: f64,
) {
    while let Some(event) = window.next() {

        match event {
            Input::Render(args) => {
                opengl.draw(args.viewport(), |context, graphics| {
                    draw(context, graphics, glyph_cache, *volume, left_alignment)
                });
            }

            // TODO: Known precision problem related to stepping f64 instead of integers.
            Input::Press(Button::Keyboard(key)) => {
                let volume_step: f64 = 0.1;

                match key {
                    Key::D => {
                        music::play_sound(&Sound::MenuSelection, music::Repeat::Times(0));
                        *volume += volume_step
                    }
                    Key::A => {
                        music::play_sound(&Sound::MenuSelection, music::Repeat::Times(0));
                        *volume -= volume_step
                    }
                    Key::Space => {
                        music::play_sound(&Sound::MenuBack, music::Repeat::Times(0));
                        break;
                    }
                    _ => {}
                }

                *volume = volume.max(music::MIN_VOLUME).min(music::MAX_VOLUME);
                music::set_volume(*volume);
            }

            _ => {}
        }
    }
}
