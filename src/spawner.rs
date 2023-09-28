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
        Health {
            current: 20,
            max: 20,
        },
    ));
}

/// Adds the monsters and their components to the ECS.
/// * `ecs` - provides mutable reference to the `World`
/// * `rng` - random number used to randomly select monster type
/// * `pos` - provides access to the monster location
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    // create components by calling `push`. The components are separated in a tuple.
    ecs.push((
        // add a `tag` component, indicating that this is an enemy
        Enemy,
        // the monsters position
        pos,
        // a `Render` component containing the monsters appearance
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
        MovingRandomly {},
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_string(), to_cp437('o'))
}
