use std::env;

// add the modules to your project with mod
mod camera;
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
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    // re-export the crates as public modules available within prelude
    pub use crate::camera::*;
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
    /// The camera.
    camera: Camera,
}

impl State {
    /// Constructor to initialise `State`.
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start),
        }
    }
}

// implements the `GameState` trait for the `State` struct
impl GameState for State {
    /// Required by `GameState`, it controls the program flow based on the
    /// current mode, and is the bridge between the game engine and the game.
    /// * `&mut self` - allows access to change the current `State` instance
    /// * `ctx` - allows access to change the currently running bracket-terminal
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }
}

/// This is the `main` function.
fn main() -> BError {
    // set RUST_BACKTRACE environment variable
    env::set_var("RUST_BACKTRACE", "full");
    // create a generic terminal and specify attributes directly
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
