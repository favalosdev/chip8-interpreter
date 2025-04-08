use super::constants::VALUE_KEY_MAP;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;

pub struct Keyboard {
    key_map: HashMap<Scancode, u8>,
    keys: [bool; 16],
    pub is_waiting_for_key: bool,
    pub last_key: Option<u8>,
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
            is_waiting_for_key: false,
            last_key: None,
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

            if self.is_waiting_for_key {
                self.last_key = Some(key);
                self.is_waiting_for_key = false;
            } else {
                println!("Normal key press: 0x{key:X} (scancode: {scancode:?})");
            }
        } else {
            println!("Received unmapped scancode: {scancode:?}");
        }
    }

    pub fn release_key(&mut self, scancode: Scancode) {
        if let Some(&key) = self.key_map.get(&scancode) {
            self.keys[key as usize] = false;
        }
    }
}
