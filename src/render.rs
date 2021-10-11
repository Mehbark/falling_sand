use crate::sim::{Element, Material, World};

pub(crate) fn start_color(material: &Material) -> u8 {
    match material {
        Material::Sand => 220,
        Material::Water => todo!(),
        Material::Wall => todo!(),
        Material::Clone => todo!(),
    }
}

pub fn render(world: &World) {
    for y in 0..world.world.len() {
        for x in 0..world.world[y].len() {
            if let Some(elem) = &world.world[y][x] {
                draw(x, y, elem.color);
            } else {
                erase(x, y);
            }
        }
    }
}

pub fn draw(x: usize, y: usize, color: u8) {
    // Put the cursor where we want to draw
    print!("\x1b[{line};{column}H", line = y / 2, column = x);

    /*
    // 0 is our erase code
    if color == 0 {
        if (y + 1) & 1 == 0 {
            // Reset the background color, but keep the top block
            print!("\x1b[49m▀", );
        } else {
            // Reset the background color, and replace
            print!("\x1b[49m▄");
        }
    } else*/ if (y + 1) & 1 == 0 { //Checks if y is even
        // Set the background color if y is even, since ▀ covers the top half
        // Note the space at the end, this is so that when we reset it won't reset our styling
        print!("\x1b[48;5;{}m▀", color);
    } else {
        // Foreground color instead, and ▀ because it will need to be used
        print!("\x1b[38;5;{}m▀", color);
    }

    // print!("\x1b[0m"); //Reset all styles, to be courteous :)
}

pub fn erase(x: usize, y: usize) {
    draw(x, y, 232);
}

pub fn sleep(time: u64) {
    std::thread::sleep(std::time::Duration::from_millis(time));
}