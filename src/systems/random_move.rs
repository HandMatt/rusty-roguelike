use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
/// Makes an entity move one space in a random direction.
/// * `ecs` - mutable access to a SubWorld (like a World - but you can only see the components requested)
/// * `map` - access a read-only reference to the map
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    // creates a query with write access to Point and read-only access to MovingRandomly
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        // randomly choose a direction to move and determine the destination
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;
        // check if the destination tile is accessible
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination,
            },
        ));
    });
}
