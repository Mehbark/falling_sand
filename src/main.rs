mod render;
mod sim;
use sim::*;

fn main() {
    let test = Element::new(0, 0, Material::Sand);
    println!("{:#?}", test);
}
