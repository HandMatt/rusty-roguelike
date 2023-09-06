use crate::prelude::*;
/// The total number of tiles in the game screen.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
/// Contains each possible tile type.
pub enum TileType {
    /// Represents a wall tile.
    Wall,
    /// Represents a floor tile.
    Floor,
}

/// The game map.
pub struct Map {
    /// The tile set.
    pub tiles: Vec<TileType>,
}

/// Calculates the tile index.
/// * `y` - the vertical position of the tile
/// * `x` - the horizontal position of the tile
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    /// Initialises the `Map`.
    pub fn new() -> Self {
        // create a map consisting entirely of floors
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Checks the location specified in point is within the screen boundaries.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Determines if the player can enter a tile.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Determines a tile's index coordinates.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
