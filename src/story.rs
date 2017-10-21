//! Displays the story dialogue.

use music;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, types, Button, Context, Key, PistonWindow, PressEvent,
                    RenderEvent, Transformed};

use game::color::{self, ColoredText};
use menu::{Sound, Volume};

fn draw(context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    const NARRATOR_COLOR: types::Color = color::WHITE;
    const KARA_COLOR: types::Color = color::MAGENTA;
    const JACK_COLOR: types::Color = color::CYAN;

    const LINES: [ColoredText; 20] = [
        ColoredText {
            color: NARRATOR_COLOR,
            text: "The stars snap back into place, \
                   jolting your neck forward.",
        },
        ColoredText {
            color: NARRATOR_COLOR,
            text: "Panicking, you check your ship’s readouts. \
                   This can’t be the right system.",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"---day, can --- read me?\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"This is Delta-Six, what is your situation?\"",
        },
        ColoredText {
            color: NARRATOR_COLOR,
            text: "A piece of twisted metal screeches off \
                   your ship’s shields.",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"Jack ---? Jack is --- you?\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"Kara, what happened here? \
                   Where’s the fleet?\"",
        },
        ColoredText {
            color: NARRATOR_COLOR,
            text: "A lifeless expanse of debris is all that \
                   surrounds you in every direction.",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"--- ambush. --- --- tried to --- long range \
                   transmitter --- --- warn ---\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"Kara, transmit me your coordinates.\"",
        },
        ColoredText {
            color: NARRATOR_COLOR,
            text: "The debris thickens, tightening \
                   its grip around you.",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"Shields --- percent.  Jack, --- --- \
                   last Ranger.\"",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"--- must relay the --- for as long \
                   as possible to --- the others.\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"Kara, where are you?!\"",
        },
        ColoredText {
            color: NARRATOR_COLOR,
            text: "Heat shoots up your spine as you thrust \
                   your engines to full.",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"Shields --- percent.  Diverting \
                   remaining --- ---\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"Kara, stay will me. I’ll find you.\"",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"--- Delta-Three --- signing off.  \
                   Jack, ... Jack, I ---\"",
        },
        ColoredText {
            color: KARA_COLOR,
            text: "\"...\"",
        },
        ColoredText {
            color: JACK_COLOR,
            text: "\"Kara!\"",
        },
    ];

    clear(color::BLACK, graphics);

    for (index, line) in LINES.iter().enumerate() {
        let left_indent = 50.0;
        let starting_line_offset = 30.0;
        let new_line_offset = 30.0;

        text(
            line.color,
            22,
            line.text,
            glyph_cache,
            context.transform.trans(
                left_indent,
                starting_line_offset + (index as f64 * new_line_offset),
            ),
            graphics,
        ).unwrap();
    }
}

/// Loop displaying the story until the user exits.
pub fn run(
    window: &mut PistonWindow,
    opengl: &mut GlGraphics,
    glyph_cache: &mut GlyphCache,
    volume: Volume,
) {
    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                draw(context, graphics, glyph_cache)
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            if key == Key::Space {
                music::play_sound(&Sound::MenuBack, music::Repeat::Times(0), volume.sound);
                break;
            }
        }
    }
}
