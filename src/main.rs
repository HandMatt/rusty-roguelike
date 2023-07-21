use bracket_lib::prelude::*;

// add the modules to your project with mod
mod map;
mod map_builder;
mod player;

// declare a new module in source code
mod prelude {
    // re-exports bracket_lib prelude inside prelude
    pub use bracket_lib::prelude::*;
    // public constants
    /// The screen width.
    pub const SCREEN_WIDTH: i32 = 80;
    /// The screen height.
    pub const SCREEN_HEIGHT: i32 = 50;
    // re-export the crates as public modules available within prelude
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

// make prelude available to the main scope
use prelude::*;

/// A snapshot of the current game.
struct State {
    /// The map.
    map: Map,
    /// The player.
    player: Player,
}

impl State {
    /// Constructor to initialise `State`.
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
        }
    }
}

// implements the `GameState` trait for the `State` struct
impl GameState for State {
    /// Required by `GameState`, it controls the program flow based on the current mode, and is the bridge between the game engine and the game.
    /// * `&mut self` - allows access to change the current `State` instance
    /// * `ctx` - allows access to change the currently running bracket-terminal
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

/// This is the `main` function.
fn main() -> BError {
    // request a simple 80x50 terminal, build it or return an BError
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
