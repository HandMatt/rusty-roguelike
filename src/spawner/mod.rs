use crate::prelude::*;
mod template;
use template::Templates;

/// Adds the player and their components to the ECS.
/// * `ecs` - provides mutable reference to the `World`
/// * `pos` - provides access to the player location
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // create components by calling `push`. The components are separated in a tuple.
    ecs.push((
        // add a `tag` component, indicating that this is the player
        Player { map_level: 0 },
        // the players position
        pos,
        // a `Render` component containing the player's appearance
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 10,
            max: 10,
        },
        FieldOfView::new(8),
        Damage(1),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYala,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yala".to_string()),
    ));
}
