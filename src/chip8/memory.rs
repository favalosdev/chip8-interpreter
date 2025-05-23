use super::constants::{FONT_DATA_START_ADDRESS, PROGRAM_START_ADDRESS};

pub struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { memory: [0; 4096] };
        // Load fontset
        for (i, &byte) in super::constants::FONT_SET.iter().enumerate() {
            memory.memory[(FONT_DATA_START_ADDRESS as usize) + i] = byte;
        }
        memory
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[(PROGRAM_START_ADDRESS as usize) + i] = byte;
        }
    }

    pub fn write_byte(&mut self, address: usize, byte: u8) {
        self.memory[address] = byte;
    }
}
