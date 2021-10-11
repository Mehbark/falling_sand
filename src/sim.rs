use crate::render::start_color;

#[derive(Debug)]
pub struct Element {
    x: usize,
    y: usize,
    material: Material,
    color: [u8; 3],
}

#[derive(Debug)]
pub enum Material {
    Sand,
    Water,
    Wall,
    Clone,
}

impl Element {
    pub fn new(x: usize, y: usize, material: Material) -> Element {
        Element {
            x,
            y,
            material,
            color: start_color(&material),
        }
    }
}
