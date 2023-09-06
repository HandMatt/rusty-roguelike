pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A Render struct describing how an entity appears on the screen.
pub struct Render {
    /// The `ColorPair` used to render the component, stores both foreground and background color in a single struct.
    pub color: ColorPair,
    /// The character or `glyph` to render.
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// A Player struct containing no data, serving as a tag.
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
/// An Enemy struct containing no data, serving as a tag.
pub struct Enemy;
