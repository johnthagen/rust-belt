//! Displays the story line.

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Context, Event, Input, Key, PistonWindow, text, types,
    Transformed};

use color;

fn render(context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    // TODO: Create a vector of tuples that map message and color and iterate.
    const LEFT_INDENT: f64 = 50.0;
    const NARRATOR_COLOR: types::Color = color::WHITE;
    const KARA_COLOR: types::Color = color::MAGENTA;
    const JACK_COLOR: types::Color = color::CYAN;
    const STARTING_LINE_OFFSET: f64 = 30.0;
    const NEW_LINE_OFFSET: f64 = 30.0;
    const FONT_SIZE: types::FontSize = 22;

    clear(color::BLACK, graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "The stars snap back into place, jolting your neck forward.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET),
         graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "Panicking, you check your ship’s readouts. \
         This can’t be the right system.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 1.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "---day, can --- read me?",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 2.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "This is Delta-Six, what is your situation?",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 3.0 * NEW_LINE_OFFSET),
         graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "A piece of twisted metal screeches off your ship’s shields.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 4.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "Jack ---? Jack is --- you?",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 5.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "Kara, what happened here? Where’s the fleet?",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 6.0 * NEW_LINE_OFFSET),
         graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "A lifeless expanse of debris is all that surrounds you in every \
         direction.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 7.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "--- ambush. --- --- tried to --- long range transmitter --- \
          --- warn ---",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 8.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "Kara, transmit me your coordinates.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 9.0 * NEW_LINE_OFFSET),
         graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "The debris thickens, tightening its grip around you.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 10.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "Shields --- percent.  Jack, --- --- last Ranger.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 11.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "--- must relay the --- for as long as possible to --- the others.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 12.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "Kara, where are you?!",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 13.0 * NEW_LINE_OFFSET),
         graphics);
    text(NARRATOR_COLOR,
         FONT_SIZE,
         "Heat shoots up your spine as you thrust your engines to full.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 14.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "Shields --- percent.  Diverting remaining --- ---",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 15.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "Kara, stay will me, I’ll find you.",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 16.0 * NEW_LINE_OFFSET),
         graphics);
    text(KARA_COLOR,
         FONT_SIZE,
         "--- Delta-Three --- signing off.  Jack, … Jack, I --- ---",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 17.0 * NEW_LINE_OFFSET),
         graphics);
    text(JACK_COLOR,
         FONT_SIZE,
         "Kara!",
         glyph_cache,
         context.transform
             .trans(LEFT_INDENT, STARTING_LINE_OFFSET +
                 18.0 * NEW_LINE_OFFSET),
         graphics);
}

pub fn run(window: &mut PistonWindow, opengl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    while let Some(event) = window.next() {
        match event {
            Event::Render(args) => {
                opengl.draw(args.viewport(),
                            |context, graphics| render(context, graphics, glyph_cache));
            }

            Event::Input(Input::Press(Button::Keyboard(key))) => {
                match key {
                    Key::Space => { break }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}