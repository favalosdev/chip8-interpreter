use super::constants::VALUE_KEY_MAP;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;

pub struct Keyboard {
    key_map: HashMap<Scancode, u8>,
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        let scan_to_hex: HashMap<Scancode, u8> = {
            let mut map = HashMap::new();
            for &(hex, scan) in &VALUE_KEY_MAP {
                map.insert(scan, hex);
            }
            map
        };

        Self {
            key_map: scan_to_hex,
            keys: [false; 16],
        }
    }

    pub fn is_key_pressed(&self, value: u8) -> bool {
        if value <= 0xF {
            return self.keys[value as usize];
        }
        false
    }

    pub fn press_key(&mut self, scancode: Scancode) {
        if let Some(&key) = self.key_map.get(&scancode) {
            self.keys[key as usize] = true;
        }
    }

    pub fn release_key(&mut self, scancode: Scancode) {
        if let Some(&key) = self.key_map.get(&scancode) {
            self.keys[key as usize] = false;
        }
    }
}
