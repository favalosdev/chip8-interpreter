use super::constants::{ORIGINAL_HEIGHT, ORIGINAL_WIDTH};

pub struct Display {
    pub pixels: [[bool; ORIGINAL_WIDTH as usize]; ORIGINAL_HEIGHT as usize],
    pub changed: bool,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; ORIGINAL_WIDTH as usize]; ORIGINAL_HEIGHT as usize],
            changed: true, // Start true to ensure initial render
        }
    }

    pub fn clear(&mut self) {
        for row in self.pixels.iter_mut() {
            row.fill(false);
        }
        self.changed = true;
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        let collision = self.pixels[y][x] && value;
        self.pixels[y][x] ^= value;
        self.changed = true;
        collision
    }
}
