use crate::prelude::*;

/// Adds the player and their components to the ECS.
/// * `ecs` - provides mutable reference to the `World`
/// * `pos` - provides access to the player location
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // create components by calling `push`. The components are separated in a tuple.
    ecs.push((
        // add a `tag` component, indicating that this is the player
        Player,
        // the players position
        pos,
        // a `Render` component containing the player's appearance
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

/// Adds the monsters and their components to the ECS.
/// * `ecs` - provides mutable reference to the `World`
/// * `rng` - random number used to randomly select monster type
/// * `pos` - provides access to the monster location
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    // create components by calling `push`. The components are separated in a tuple.
    ecs.push((
        // add a `tag` component, indicating that this is an enemy
        Enemy,
        // the monsters position
        pos,
        // a `Render` component containing the monsters appearance
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                // ettin
                0 => to_cp437('E'),
                // ogre
                1 => to_cp437('O'),
                // orc
                2 => to_cp437('o'),
                // goblin
                _ => to_cp437('g'),
            },
        },
        MovingRandomly {},
    ));
}
