use crate::prelude::*;

/// The `Player`.
pub struct Player {
    /// The x and y position of the player.
    pub position: Point,
}

impl Player {
    /// Constructor to initialise `Player`
    /// * `position` - the 2D starting position
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// Check for player input and calculate the new intended position, update
    /// the player position if the move is valid.
    /// * `&mut self` - allows access to change the current `Player` instance
    /// * `ctx` - allows access to change the currently running bracket-terminal
    /// * `map` - allows access to the `Map` instance
    /// * `camera` - allows access to change the `Camera` instance
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;
            if map.can_enter_tile(new_position) {
                self.position = new_position;
                camera.on_player_move(new_position);
            }
        }
    }

    /// Render the player.
    /// * `&self` - allows access to the current `Player` instance
    /// * `ctx` - allows access to change the currently running bracket-terminal
    /// * `camera` - allows access to change the `Camera` instance
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }
}
