mod render;
mod sim;

use render::*;
use sim::*;

fn main() {
    let world = World::new(81, 81);
std::thread::sleep(std::time::Duration::from_secs(5));
    world.render();
}



