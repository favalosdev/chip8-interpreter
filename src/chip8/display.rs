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

    #[cfg(test)]
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_initialization() {
        let display = Display::new();
        assert!(display.changed);
        for y in 0..ORIGINAL_HEIGHT as usize {
            for x in 0..ORIGINAL_WIDTH as usize {
                assert!(!display.pixels[y][x]);
            }
        }
    }

    #[test]
    fn test_clear_display() {
        let mut display = Display::new();
        display.pixels[0][0] = true;
        display.changed = false;

        display.clear();

        assert!(!display.pixels[0][0]);
        assert!(display.changed);
    }

    #[test]
    fn test_draw_pixel() {
        let mut display = Display::new();

        // Test setting a pixel
        assert!(!display.draw_pixel(0, 0, true));
        assert!(display.get_pixel(0, 0));
        assert!(display.changed);

        // Test collision detection
        display.changed = false;
        assert!(display.draw_pixel(0, 0, true));
        assert!(!display.get_pixel(0, 0));
        assert!(display.changed);
    }
}
