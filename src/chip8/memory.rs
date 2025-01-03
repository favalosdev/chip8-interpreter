pub struct Memory {
    memory: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { memory: [0; 4096] };
        // Load fontset
        for (i, &byte) in super::constants::FONT_SET.iter().enumerate() {
            memory.memory[0x50 + i] = byte;
        }
        memory
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }
}
