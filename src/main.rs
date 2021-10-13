mod render;
mod sim;

use render::*;
use sim::*;

fn main() {
    let mut world = World::new(80, 80);
    world.spawn_element(0, 0, Element::new(0, 0, Material::Sand));
    world.spawn_element(0, 3, Element::new(0, 3, Material::Sand));
    println!("{:?}", world.world[0][0].unwrap());
    // draw(0, 0, world.world[0][0].unwrap().color);
    // erase(0, 2);
    // world.tick();
    world.render();
    // draw(0, 0, 220);
    // erase(0, 2);
}



