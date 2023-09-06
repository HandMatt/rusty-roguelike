use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
/// Renders entities with both a Point and Render component.
/// * `ecs` - access to a SubWorld (like a World - but you can only see the components requested)
/// * `camera` - gives access to the camera resource
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });
    draw_batch.submit(5000).expect("Batch error");
}
