pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A render struct describing how an entity appears on the screen.
pub struct Render {
    /// The `ColorPair` used to render the component, stores both foreground and background color in a single struct.
    pub color: ColorPair,
    /// The character or `glyph` to render.
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// A player struct containing no data, serving as a tag.
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
/// An enemy struct containing no data, serving as a tag.
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A random movement struct containing no data, serving as a tag.
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
// An intent to move struct, has a reference to an entity and a location
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}
