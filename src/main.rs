use std::env;

// add the modules to your project with mod
mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

// declare a new module in source code
mod prelude {
    // publicly re-export the crates bracket_lib::prelude
    // and Legion making them available within prelude
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    // public constants available within prelude
    /// The screen width.
    pub const SCREEN_WIDTH: i32 = 80;
    /// The screen height.
    pub const SCREEN_HEIGHT: i32 = 50;
    /// The display width.
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    /// The display height.
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    // re-export the crates as public modules available within prelude
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

// make prelude available to the main scope
use prelude::*;

/// A snapshot of the current game.
struct State {
    /// Storage for all entities and components through the `legion` world structure.
    ecs: World,
    /// Storage for Map and Camera resources.
    resources: Resources,
    /// Storage for the games systems.
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    /// Initialises the game `State`.
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }
}

// implements the `GameState` trait for the `State` struct
impl GameState for State {
    /// Controls the program flow based on the current mode, and is the bridge
    /// between the game engine and the game.
    /// * `&mut self` - allows access to change the current `State` instance
    /// * `ctx` - allows access to change the currently running `bracket_terminal`
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

/// This is the `main` function.
fn main() -> BError {
    // set RUST_BACKTRACE environment variable
    env::set_var("RUST_BACKTRACE", "full");
    // use new to create a generic terminal and specify attributes directly
    let context = BTermBuilder::new()
        .with_title("Rusty Roguelike")
        // fps_cap automatically tracks game speed and tells the OS it can rest in between frames
        .with_fps_cap(30.0)
        // with_dimensions specifies the size of subsequent consoles
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // the tile dimensions are the size of each character in the font file
        .with_tile_dimensions(32, 32)
        // the directory containing the graphics file
        .with_resource_path("resources/")
        // the name of the font file to load and the dimensions of the characters
        .with_font("dungeonfont.png", 32, 32)
        // add a console using the dimensions specified and the named graphics file
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // add a second console with no background, so transparency shows through it
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
