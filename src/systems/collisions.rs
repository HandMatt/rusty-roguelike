use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
/// Detects when a collision occurs between the player character and an enemy.
/// * `ecs` - mutable access to a SubWorld (like a World - but you can only see the components requested)
/// * `commands` - allows access to the CommandBuffer from `legion`  
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    enemies
        .iter(ecs)
        // filter only positions that match the players
        .filter(|(_, pos)| **pos == player_pos)
        // the first tuple entry is the Entity position is no longer needed 
        .for_each(|(entity, _)| {
            // remove the specified entity from the world at the end of the frame
            commands.remove(*entity);
        });
}
