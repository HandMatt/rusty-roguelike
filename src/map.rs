use crate::prelude::*;
/// Number for tiles in the game screen.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Clone adds a clone function to the type
// Copy changes the default action when assigning a TileType one variable to another
// PartialEq adds code that allows == comparisons
#[derive(Copy, Clone, PartialEq)]
/// Contains each possible tile type.
pub enum TileType {
    Wall,
    Floor,
}

/// The game map.
pub struct Map {
    /// The tile set.
    pub tiles: Vec<TileType>,
}

/// Calculates the tile index.
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    /// Constructor to initialise `Map`.
    pub fn new() -> Self {
        // create a map consisting entirely of floors
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    /// Use for loops to iterate through the y and x values of the map, 
    /// uses match to determine tile type, and calls set to render each map tile.
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TileType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}
