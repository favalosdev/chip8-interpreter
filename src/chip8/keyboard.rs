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

        let mappings = [
            (0x0, Scancode::X),
            (0x1, Scancode::Num1),
            (0x2, Scancode::Num2),
            (0x3, Scancode::Num3),
            (0x4, Scancode::Q),
            (0x5, Scancode::W),
            (0x6, Scancode::E),
            (0x7, Scancode::A),
            (0x8, Scancode::S),
            (0x9, Scancode::D),
            (0xA, Scancode::Z),
            (0xB, Scancode::C),
            (0xC, Scancode::Num4),
            (0xD, Scancode::R),
            (0xE, Scancode::F),
            (0xF, Scancode::V),
        ];

        for &(hex, scan) in &mappings {
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

        // Should never happen
        return 0;
    }
}
