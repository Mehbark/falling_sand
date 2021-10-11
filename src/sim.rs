use crate::render::{render, start_color};

#[derive(Debug)]
pub struct Element {
    pub x: usize,
    pub y: usize,
    pub material: Material,
    pub color: u8,
}

#[derive(Debug)]
pub enum Material {
    Sand,
    Water,
    Wall,
    Clone,
}

impl Element {
    pub fn new(x: usize, y: usize, material: Material) -> Self {
        Self {
            x,
            y,
            color: start_color(&material),
            material,
        }
    }
}

#[derive(Debug)]
pub struct World {
    pub height: usize,
    pub width: usize,
    pub world: Vec<Vec<Option<Element>>>,
}

impl World {
    pub fn new(height: usize, width: usize) -> Self {
        let world: Vec<Vec<Option<Element>>> = (0..height)
            .map(|_| (0..width).map(|_| None).collect())
            .collect();
        Self { height, width, world }
    }
}

impl World {
    pub fn render(&self) {
        render(self);
    }
}