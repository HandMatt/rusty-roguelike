use crate::prelude::*;

/// The game camera.
pub struct Camera {
    /// Camera view left boundary.
    pub left_x: i32,
    /// Camera view right boundary.
    pub right_x: i32,
    /// Camera view top boundary.
    pub top_y: i32,
    /// Camera view bottom boundary.
    pub bottom_y: i32,
}

impl Camera {
    /// Creates a `Camera` and centres it on the player.
    /// * `player_position` - the player character's coordinate position
    pub fn new(player_position: Point) -> Self {
        Self {
            left_x: player_position.x - DISPLAY_WIDTH / 2,
            right_x: player_position.x + DISPLAY_WIDTH / 2,
            top_y: player_position.y - DISPLAY_HEIGHT / 2,
            bottom_y: player_position.y + DISPLAY_HEIGHT / 2,
        }
    }

    /// Ensures the camera remains centred on the player position.
    /// * `&mut self` - allows access to change the current `Camera` instance
    /// * `player_position` - the player character's coordinate position
    pub fn on_player_move(&mut self, player_position: Point) {
        self.left_x = player_position.x - DISPLAY_WIDTH / 2;
        self.right_x = player_position.x + DISPLAY_WIDTH / 2;
        self.top_y = player_position.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = player_position.y + DISPLAY_HEIGHT / 2;
    }
}
