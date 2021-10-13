use crate::render::{render, start_color};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Element {
    pub x: usize,
    pub y: usize,
    pub material: Material,
    pub color: u8,
}

#[derive(Clone, Copy)]
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

impl Element {
    pub fn sand_tick(&self, world: &mut World) {
        if world.is_empty(self.x, self.y + 1) {
            world.move_element(self.x, self.y, self.x, self.y + 1);
        } else if self.x != 0 && world.is_empty(self.x - 1, self.y + 1) {
            world.move_element(self.x, self.y, self.x - 1, self.y + 1);
        } else if world.is_empty(self.x + 1, self.y + 1) {
            world.move_element(self.x, self.y, self.x + 1, self.y + 1);
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

    pub fn tick(&mut self) {
        for y in 0..self.world.len() {
            for x in 0..self.world[y].len() {
                if let Some(elem) = self.world[y][x] { self.decide_fun(elem) }
            }
        }
    }

    pub fn decide_fun(&mut self, elem: Element) {
        match elem.material {
            Material::Sand => elem.sand_tick(self),
            Material::Water => todo!(),
            Material::Wall => todo!(),
            Material::Clone => todo!(),
        }
    }

    pub fn move_element(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> bool {
        if self.is_empty(to_x, to_y) && !self.is_empty(from_x, from_y) {
            self.world[to_y][to_x] = Some(self.world[from_y][from_x].expect("Uh oh this is supposed to exist"));
            self.world[from_y][from_x] = None;
            self.world[to_y][to_x].unwrap().x = to_x;
            self.world[to_y][to_x].unwrap().y = to_y;
            return true;
        }
        false
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.world[y][x].is_none()
    }

    pub fn spawn_element(&mut self, x: usize, y: usize, elem: Element) -> bool {
        if self.is_empty(x, y) {
            self.world[y][x] = Some(elem);
            return true;
        }
        false
    }
}