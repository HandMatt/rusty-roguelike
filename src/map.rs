use crate::prelude::*;
/// Number for tiles in the game screen.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

// Clone adds a clone function to the type
// Copy changes the default action when assigning TileType variable to another
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

    /// Checks the location specified in point is within the screen boundaries.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /// Determine if the player can enter a tile.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /// Determine a tile's index coordinates.
    /// * `&self` - allows access to the current `Map` instance
    /// * `point` - the 2D position of the tile
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    /// Renders the map using for loops to iterate through the y and x values
    /// of the camera boundaries, then uses match to determine tile type, and calls set to
    /// render each map tile.
    /// * `&self` - allows access to the current `Map` instance
    /// * `ctx` - allows to change the currently running bracket-terminal
    /// * `camera` - allows access to the `Camera` instance
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if self.in_bounds(Point::new(x, y)) {
                    let idx = map_idx(x, y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                }
            }
        }
    }
}
