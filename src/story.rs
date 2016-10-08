//! Displays the story line.

use std;
use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, text, types,
    Transformed};

use color;

pub fn run(window: &mut PistonWindow, font_file: &std::path::PathBuf) {
    // TODO: Find a way to pass in the GlyphCache directly instead of the path to the font file.
    // GlyphCache is located in piston2d-gfx_graphics.
    let mut glyph_cache = Glyphs::new(font_file, window.factory.clone()).unwrap();
    while let Some(event) = window.next() {
        // TODO: Create a vector of tuples that map message and color and iterate.
        const LEFT_INDENT: f64 = 50.0;
        const NARRATOR_COLOR: types::Color = color::WHITE;
        const KARA_COLOR: types::Color = color::MAGENTA;
        const JACK_COLOR: types::Color = color::CYAN;
        const STARTING_LINE_OFFSET: f64 = 30.0;
        const NEW_LINE_OFFSET: f64 = 30.0;
        const FONT_SIZE: u32 = 22;

        window.draw_2d(&event,
                       |context, graphics| {
                           clear(color::BLACK, graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "The stars snap back into place, jolting your neck forward.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET),
                                graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "Panicking, you check your ship’s readouts. \
                                This can’t be the right system.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        1.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "---day, can --- read me?",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        2.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "This is Delta-Six, what is your situation?",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        3.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "A piece of twisted metal screeches off your ship’s shields.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        4.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "Jack ---? Jack is --- you?",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        5.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "Kara, what happened here? Where’s the fleet?",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        6.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "A lifeless expanse of debris is all that surrounds you in every \
                                 direction.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        7.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "--- ambush. --- --- tried to --- long range transmitter --- \
                                --- warn ---",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        8.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "Kara, transmit me your coordinates.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        9.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "The debris thickens, tightening its grip around you.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        10.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "Shields --- percent.  Jack, --- --- last Ranger.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        11.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "--- must relay the --- for as long as possible to --- the others.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        12.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "Kara, where are you?!",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        13.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(NARRATOR_COLOR,
                                FONT_SIZE,
                                "Heat shoots up your spine as you thrust your engines to full.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        14.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "Shields --- percent.  Diverting remaining --- ---",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        15.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "Kara, stay will me, I’ll find you.",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        16.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(KARA_COLOR,
                                FONT_SIZE,
                                "--- Delta-Three --- signing off.  Jack, … Jack, I --- ---",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        17.0 * NEW_LINE_OFFSET),
                                graphics);
                           text(JACK_COLOR,
                                FONT_SIZE,
                                "Kara!",
                                &mut glyph_cache,
                                context.transform
                                    .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                                        18.0 * NEW_LINE_OFFSET),
                                graphics);
                       });

        if let Some(button) = event.press_args() {
            match button {
                Button::Keyboard(Key::Space) => { break }
                _ => {}
            }
        }
    }
}