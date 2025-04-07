use super::constants::VALUE_KEY_MAP;
use sdl2::keyboard::Scancode;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Keyboard {
    key_map: HashMap<Scancode, u8>,
    keys: [bool; 16],
    pub is_waiting_for_key: bool,
    key_sender: Sender<u8>,
    pub last_key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new(key_sender: Sender<u8>) -> Self {
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
            key_sender,
            last_key_pressed: None,
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
            /*
            if self.is_waiting_for_key {
                println!("Key 0x{key:X} pressed during key-wait state");
                self.last_key_pressed = Some(key);
                println!("Stored key 0x{key:X} as last pressed key");
                println!("Exiting key-wait state");
                self.is_waiting_for_key = false
            } else {
                println!("Normal key press: 0x{key:X} (scancode: {scancode:?})");
            }
            */
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
