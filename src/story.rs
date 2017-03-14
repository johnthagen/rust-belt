//! Displays the story dialogue.

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Context, Input, Key, PistonWindow, text, Transformed, types};

use game::color::{self, ColoredText};

fn draw(context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    let narrator_color: types::Color = color::WHITE;
    let kara_color: types::Color = color::MAGENTA;
    let jack_color: types::Color = color::CYAN;

    const LINES: [ColoredText; 20] = [ColoredText {
                                          color: narrator_color,
                                          text: "The stars snap back into place, \
                                          jolting your neck forward.",
                                      },
                                      ColoredText {
                                          color: narrator_color,
                                          text: "Panicking, you check your ship’s readouts. \
                                                This can’t be the right system.",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"---day, can --- read me?\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"This is Delta-Six, what is your situation?\"",
                                      },
                                      ColoredText {
                                          color: narrator_color,
                                          text: "A piece of twisted metal screeches off \
                                          your ship’s shields.",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"Jack ---? Jack is --- you?\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"Kara, what happened here? \
                                          Where’s the fleet?\"",
                                      },
                                      ColoredText {
                                          color: narrator_color,
                                          text: "A lifeless expanse of debris is all that \
                                          surrounds you in every direction.",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"--- ambush. --- --- tried to --- long range \
                                          transmitter --- --- warn ---\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"Kara, transmit me your coordinates.\"",
                                      },
                                      ColoredText {
                                          color: narrator_color,
                                          text: "The debris thickens, tightening \
                                          its grip around you.",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"Shields --- percent.  Jack, --- --- \
                                          last Ranger.\"",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"--- must relay the --- for as long \
                                          as possible to --- the others.\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"Kara, where are you?!\"",
                                      },
                                      ColoredText {
                                          color: narrator_color,
                                          text: "Heat shoots up your spine as you thrust \
                                          your engines to full.",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"Shields --- percent.  Diverting \
                                          remaining --- ---\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"Kara, stay will me. I’ll find you.\"",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"--- Delta-Three --- signing off.  \
                                          Jack, ... Jack, I ---\"",
                                      },
                                      ColoredText {
                                          color: kara_color,
                                          text: "\"...\"",
                                      },
                                      ColoredText {
                                          color: jack_color,
                                          text: "\"Kara!\"",
                                      }];

    clear(color::BLACK, graphics);

    for (index, line) in LINES.iter().enumerate() {
        let left_indent = 50.0;
        let starting_line_offset = 30.0;
        let new_line_offset = 30.0;

        text(line.color,
             22,
             line.text,
             glyph_cache,
             context.transform.trans(left_indent,
                                     starting_line_offset + (index as f64 * new_line_offset)),
             graphics);
    }
}

/// Loop displaying the story until the user exits.
pub fn run(window: &mut PistonWindow, opengl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    while let Some(event) = window.next() {
        match event {
            Input::Render(args) => {
                opengl.draw(args.viewport(),
                            |context, graphics| draw(context, graphics, glyph_cache));
            }

            Input::Press(Button::Keyboard(key)) => {
                if key == Key::Space {
                    break;
                }
            }

            _ => {}
        }
    }
}
