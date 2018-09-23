//! Defines common, reusable colors.

use piston_window::types;

crate const BLACK: types::Color = [0.0, 0.0, 0.0, 1.0];
crate const WHITE: types::Color = [1.0, 1.0, 1.0, 1.0];
crate const CYAN: types::Color = [0.0, 1.0, 1.0, 1.0];
crate const YELLOW: types::Color = [1.0, 1.0, 0.0, 1.0];
crate const MAGENTA: types::Color = [1.0, 0.0, 1.0, 1.0];
crate const DIM_RED: types::Color = [1.0, 0.0, 0.0, 0.5];

/// A type for storing text and an associated color it should
/// be drawn as.
crate struct ColoredText {
    crate color: types::Color,
    crate text: &'static str,
}
