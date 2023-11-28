pub use crate::prelude::*;
use std::collections::HashSet;

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
pub struct Player {
    pub map_level: u32,
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}
