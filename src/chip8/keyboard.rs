use sdl2::keyboard::Scancode;
use std::collections::HashMap;

pub struct Keyboard {
    key_map: HashMap<Scancode, usize>,
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        let mut key_map = HashMap::new();

        key_map.insert(Scancode::Num1, 0x1);
        key_map.insert(Scancode::Num2, 0x2);
        key_map.insert(Scancode::Num3, 0x3);
        key_map.insert(Scancode::Num4, 0xC);
        key_map.insert(Scancode::Q, 0x4);
        key_map.insert(Scancode::W, 0x5);
        key_map.insert(Scancode::E, 0x6);
        key_map.insert(Scancode::R, 0xD);
        key_map.insert(Scancode::A, 0x7);
        key_map.insert(Scancode::S, 0x8);
        key_map.insert(Scancode::D, 0x9);
        key_map.insert(Scancode::F, 0xE);
        key_map.insert(Scancode::Z, 0xA);
        key_map.insert(Scancode::X, 0x0);
        key_map.insert(Scancode::C, 0xB);
        key_map.insert(Scancode::V, 0xF);

        Self {
            key_map,
            keys: [false; 16],
        }
    }

    pub fn press_key(&mut self, scancode: Scancode) {
        if let Some(&key) = self.key_map.get(&scancode) {
            self.keys[key] = true;
        }
    }

    pub fn release_key(&mut self, scancode: Scancode) {
        if let Some(&key) = self.key_map.get(&scancode) {
            self.keys[key] = false;
        }
    }
}
