use bracket_lib::prelude::*;

// add the module to your project with mod
mod map;

// declare a new module in source code
mod prelude {
    // re-exports bracket_lib prelude inside prelude
    pub use bracket_lib::prelude::*;
    // public constants
    /// The screen width.
    pub const SCREEN_WIDTH: i32 = 80;
    /// The screen height.
    pub const SCREEN_HEIGHT: i32 = 50;
    // re-export the map as a public module available within prelude
    pub use crate::map::*;
}

// make prelude available to the main scope
use prelude::*;

/// A snapshot of the current game.
struct State {
    /// The map.
    map: Map,
}

impl State {
    /// Constructor to initialise `State`.
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

// implements the `GameState` trait for the `State` struct
impl GameState for State {
    /// Required by `GameState`, it controls the program flow based on the current mode, and is the bridge between the game engine and the game.
    /// * `&mut self` - allows access to change the `State` instance
    /// * `ctx` - allows access to change the currently running bracket-terminal
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx);
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
