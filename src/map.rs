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

    // Determines if an exit from a tile is valid.
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        // calculate the destination by adding the current position to the delta
        let destination = loc + delta;
        // check that the destination is on the map
        if self.in_bounds(destination) {
            // determine if you can enter the tile
            if self.can_enter_tile(destination) {
                // if you can enter the tile determine its array index and return Some(idx)
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                // if you are not able to enter the tile return None
                None
            }
        } else {
            None
        }
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}
