//! Modify game settings.

use music;
use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Input, Key, PistonWindow, text, Transformed};

use game::color;

pub fn run(window: &mut PistonWindow,
           opengl: &mut GlGraphics,
           glyph_cache: &mut GlyphCache,
           volume: &mut f64,
           left_alignment: f64) {
    let value_left_alignment = left_alignment + 300.0;

    while let Some(event) = window.next() {
        const STARTING_LINE_OFFSET: f64 = 280.0;

        match event {
            Input::Render(args) => {
                opengl.draw(args.viewport(), |context, graphics| {
                    clear(color::BLACK, graphics);
                    text(color::WHITE,
                         32,
                         "Volume",
                         glyph_cache,
                         context.transform
                             .trans(left_alignment, STARTING_LINE_OFFSET),
                         graphics);
                    text(color::WHITE,
                         32,
                         &format!("{}%", ((*volume) * 100.0) as i32),
                         glyph_cache,
                         context.transform
                             .trans(value_left_alignment, STARTING_LINE_OFFSET),
                         graphics);
                });
            }

            // TODO: Known precision problem related to stepping f64 instead of integers.
            Input::Press(Button::Keyboard(key)) => {
                const VOLUME_STEP: f64 = 0.1;

                match key {
                    Key::D => *volume += VOLUME_STEP,
                    Key::A => *volume -= VOLUME_STEP,
                    Key::Space => break,
                    _ => {}
                }

                *volume = volume.max(music::MIN_VOLUME).min(music::MAX_VOLUME);
                music::set_volume(*volume);
            }

            _ => {}
        }
    }
}
