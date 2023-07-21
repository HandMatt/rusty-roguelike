use crate::prelude::*;
// Maximum number of rooms in the dungeon.
const NUM_ROOMS: usize = 20;

/// The map builder.
pub struct MapBuilder {
    /// The game map.
    pub map: Map,
    /// The rooms to be generated.
    pub rooms: Vec<Rect>,
    /// The player start location.
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    /// Fills the map with the tile specified.
    /// * `&mut self` - allows access to change the current `MapBuilder` instance
    /// * `tile` - the desired `TileType` to fill the map with
    fn fill(&mut self, tile: TileType) {
        // obtain mutable iterator, then change every tile
        // the * before the t is a de-reference, making sure we write to
        // the referenced variable, not the reference itself
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /// Randomly place rooms on the map.
    /// * `&mut self` - allows access to change the current `MapBuilder` instance
    /// * `rng` - allows access to the RandomNumberGenerator from bracket_random
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // keep generating rooms until there are NUM_ROOMS rooms on the map
        while self.rooms.len() < NUM_ROOMS {
            // generate randomly positioned rooms of random sizes
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            // test the new room against each placed room,
            // and flag it as overlapping if rooms intersect
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            // if the rooms don't overlap, check that they are within
            // the map boundaries and set their contents to floors
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    /// Create a vertical tunnel between two points on the map.
    /// * `&mut self` - allows access to change the current `MapBuilder` instance
    /// * `y1` - vertical start point of the tunnel
    /// * `y2` - vertical end point of the tunnel
    /// * `x` - horizontal position of the tunnel
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Create a horizontal tunnel between two points on the map.
    /// * `&mut self` - allows access to change the current `MapBuilder` instance
    /// * `x1` - horizontal start point of the tunnel
    /// * `x2` - horizontal end point of the tunnel
    /// * `y` - vertical position of the tunnel
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Generate complete corridors between rooms.
    /// * `&mut self` - allows access to change the current `MapBuilder` instance
    /// * `rng` - allows access to the RandomNumberGenerator from bracket_random
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // sort rooms by their center point to make sure adjacent rooms join to one another
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        // iterate through the rooms, ignoring the first one in the iterator
        for (i, room) in rooms.iter().enumerate().skip(1) {
            // obtain the center position of the current and previous room
            let prev = rooms[i - 1].center();
            let new = room.center();

            // randomly dig the horizontal and then vertical parts of the corridor, or vice versa
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}
