use super::constants::VALUE_KEY_MAP;
use sdl2::{event::Event, keyboard::Scancode, EventPump};
use std::collections::HashMap;

pub struct Keyboard {
    hex_to_scan: HashMap<u8, Scancode>,
    scan_to_hex: HashMap<Scancode, u8>,
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(event_pump: EventPump) -> Self {
        let mut hex_to_scan = HashMap::new();
        let mut scan_to_hex = HashMap::new();

        for &(hex, scan) in &VALUE_KEY_MAP {
            hex_to_scan.insert(hex, scan);
            scan_to_hex.insert(scan, hex);
        }

        Self {
            hex_to_scan,
            scan_to_hex,
            event_pump,
        }
    }

    pub fn is_key_pressed(&self, value: u8) -> bool {
        if value <= 0xF {
            return self.hex_to_scan.get(&value).map_or(false, |p| {
                self.event_pump.keyboard_state().is_scancode_pressed(*p)
            });
        }
        false
    }

    pub fn wait_until_press(&mut self) -> u8 {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    let code = self.scan_to_hex.get(&scancode).unwrap_or(&0);
                    return *code;
                }
                _ => {}
            }
        }

        return 0;
    }
}
