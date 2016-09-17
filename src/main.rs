extern crate piston_window;
extern crate find_folder;

use piston_window::{Button, clear, Glyphs, Key, PressEvent, PistonWindow, polygon, text, types,
    Transformed, WindowSettings};

mod color {
    use piston_window::{types};

    pub const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
    pub const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];
    pub const CYAN: types::Color = [0.0, 1.0, 1.0, 1.0];
    pub const YELLOW: types::Color = [1.0, 1.0, 0.0, 1.0];
    pub const MAGENTA: types::Color = [1.0, 0.0, 1.0, 1.0];
}

const GAME_TITLE: &'static str = "Rust Belt";
const GAME_WINDOW_WIDTH: u32 = 1024;
const GAME_WINDOW_HEIGHT: u32 = 768;


fn main() {
    let mut window: PistonWindow = WindowSettings::new(GAME_TITLE,
                                                       [GAME_WINDOW_WIDTH, GAME_WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|error| { panic!("Failed to build PistonWindow: {}", error) });

    Menu {
        menu_selection: MenuSelection::Play,
    }.run(&mut window);
}

enum MenuSelection {
    Play,
    Story,
    Exit
}

/// Stores Menu state.
struct Menu {
    menu_selection: MenuSelection,
}

impl Menu {
    fn run(&mut self, mut window: &mut PistonWindow) {
        let assets_folder = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let ref font_file = assets_folder.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let mut glyph_cache = Glyphs::new(font_file, factory).unwrap();

        while let Some(event) = window.next() {
            const MENU_ALIGN: f64 = ((GAME_WINDOW_WIDTH / 2) - 120) as f64;
            const STARTING_LINE_OFFSET: f64 = 280.0;
            const NEW_LINE_OFFSET: f64 = 40.0;

            // TODO: Can this be done better with 'if let' ?
            let mut play_color = color::WHITE;
            let mut story_color = color::WHITE;
            let mut exit_color = color::WHITE;
            match self.menu_selection {
                MenuSelection::Play => { play_color = color::YELLOW }
                MenuSelection::Story => { story_color = color::YELLOW }
                MenuSelection::Exit => { exit_color = color::YELLOW }
            }

            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               text(color::WHITE,
                                    72,
                                    GAME_TITLE,
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET),
                                    graphics);
                               text(play_color,
                                    32,
                                    "Play",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            1.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(story_color,
                                    32,
                                    "Story",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            2.0 * NEW_LINE_OFFSET),
                                    graphics);
                               text(exit_color,
                                    32,
                                    "Exit",
                                    &mut glyph_cache,
                                    context.transform
                                        .trans(MENU_ALIGN, STARTING_LINE_OFFSET +
                                            3.0 * NEW_LINE_OFFSET),
                                    graphics);
                           });

            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::W) => {
                        match self.menu_selection {
                            MenuSelection::Play => {}
                            MenuSelection::Story => { self.menu_selection = MenuSelection::Play }
                            MenuSelection::Exit => { self.menu_selection = MenuSelection::Story }
                        }
                    }
                    Button::Keyboard(Key::S) => {
                        match self.menu_selection {
                            MenuSelection::Play => { self.menu_selection = MenuSelection::Story }
                            MenuSelection::Story => { self.menu_selection = MenuSelection::Exit }
                            MenuSelection::Exit => {}
                        }
                    }
                    Button::Keyboard(Key::Space) => {
                        match self.menu_selection {
                            MenuSelection::Play => {
                                Game {
                                    position: Position { x: 0.0, y: 0.0 },
                                    rotation: 0.0,
                                }.run(&mut window);
                            }
                            MenuSelection::Story => {
                                run_story(&mut window, font_file);
                            }
                            MenuSelection::Exit => { break }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn run_story(window: &mut PistonWindow, font_file: &std::path::PathBuf) {
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
        const NEW_LINE_OFFSET: f64 = 40.0;
        const FONT_SIZE: u32 = 24;

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

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static [[f64; 2]] = &[
    [0.0, -1.0 * SHIP_HEIGHT / 2.0],
    [SHIP_WIDTH, 0.0],
    [0.0, SHIP_HEIGHT / 2.0]
];

struct Position {
    x: f64,
    y: f64,
}

/// Stores Game state.
struct Game {
    position: Position,
    rotation: f64,
}

impl Game {
    fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               polygon(color::CYAN,
                                       SHIP,
                                       context.transform
                                           .trans(self.position.x,
                                                  self.position.y)
                                           .rot_rad(self.rotation)
                                           // Without this trans(), rotation occurs around the
                                           // upper left corner rather than the center.
                                           .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                       graphics);
                           });
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::D) => { self.position.x += 1.0 }
                    Button::Keyboard(Key::A) => { self.position.x -= 1.0 }
                    Button::Keyboard(Key::S) => { self.position.y += 1.0 }
                    Button::Keyboard(Key::W) => { self.position.y -= 1.0 }
                    Button::Keyboard(Key::Q) => { self.rotation -= 0.1 }
                    Button::Keyboard(Key::E) => { self.rotation += 0.1 }
                    Button::Keyboard(Key::X) => { break }
                    _ => {}
                }
            }
        }
    }
}