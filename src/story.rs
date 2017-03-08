//! Displays the story line.

use opengl_graphics::GlGraphics;
use opengl_graphics::glyph_cache::GlyphCache;
use piston_window::{Button, clear, Context, Input, Key, PistonWindow, text, Transformed, types};

use game::color::{self, ColoredText};

fn render(context: Context, graphics: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    const NARRATOR_COLOR: types::Color = color::WHITE;
    const KARA_COLOR: types::Color = color::MAGENTA;
    const JACK_COLOR: types::Color = color::CYAN;

    const LINES: [ColoredText; 20] = [ColoredText {
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
                                      }];

    clear(color::BLACK, graphics);

    for (index, line) in LINES.iter().enumerate() {
        const LEFT_INDENT: f64 = 50.0;
        const STARTING_LINE_OFFSET: f64 = 30.0;
        const NEW_LINE_OFFSET: f64 = 30.0;

        text(line.color,
             22,
             line.text,
             glyph_cache,
             context.transform.trans(LEFT_INDENT,
                                     STARTING_LINE_OFFSET + (index as f64 * NEW_LINE_OFFSET)),
             graphics);
    }
}

pub fn run(window: &mut PistonWindow, opengl: &mut GlGraphics, glyph_cache: &mut GlyphCache) {
    while let Some(event) = window.next() {
        match event {
            Input::Render(args) => {
                opengl.draw(args.viewport(),
                            |context, graphics| render(context, graphics, glyph_cache));
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
