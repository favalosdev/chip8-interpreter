use super::{constants::*, display::Display, memory::Memory};

pub struct CPU {
    // Program counter
    pc: u16,
    // General purpose registers V0-VF
    v: [u8; 16],
    // Index register
    i: u16,
    // Stack for subroutines
    stack: Vec<u16>,
    // Timers
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x200, // Program starts at 0x200
            v: [0; 16],
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn step(&mut self, memory: &mut Memory, display: &mut Display) -> Result<(), String> {
        // Fetch
        let opcode = self.fetch(memory);
        // Decode and Execute
        self.execute(opcode, memory, display)
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        let high_byte = memory.read_byte(self.pc as usize) as u16;
        let low_byte = memory.read_byte((self.pc + 1) as usize) as u16;
        (high_byte << 8) | low_byte
    }

    fn execute(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut Display,
    ) -> Result<(), String> {
        // Decode opcode parts
        let op_1 = ((opcode & 0xF000) >> 12) as u8;
        let op_2 = ((opcode & 0x0F00) >> 8) as u8;
        let op_3 = ((opcode & 0x00F0) >> 4) as u8;
        let op_4 = (opcode & 0x000F) as u8;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let x = op_2 as usize;
        let y = op_3 as usize;

        // Increment PC by default (some instructions will override this)
        self.pc += 2;

        match op_1 {
            0x0 => {
                match opcode {
                    0x00E0 => display.clear(),
                    0x00EE => {
                        // Return from subroutine
                        self.pc = self.stack.pop().ok_or("Stack underflow")?;
                    }
                    _ => return Err(format!("Unknown opcode: {:#06X}", opcode)),
                }
            }
            0x1 => {
                // Jump to address
                self.pc = nnn;
            }
            0x6 => {
                // Set VX to NN
                self.v[x] = nn;
            }
            0x7 => {
                // Add NN to VX
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0xA => {
                // Set I to NNN
                self.i = nnn;
            }
            0xD => {
                // Display/draw
                self.v[0xF] = 0;
                let x_coord = self.v[x] as usize % ORIGINAL_WIDTH as usize;
                let y_coord = self.v[y] as usize % ORIGINAL_HEIGHT as usize;
                let height = op_4 as usize;

                for row in 0..height {
                    if y_coord + row >= ORIGINAL_HEIGHT as usize {
                        break;
                    }

                    let sprite_byte = memory.read_byte(self.i as usize + row);

                    for col in 0..SPRITE_WIDTH {
                        if x_coord + col >= ORIGINAL_WIDTH as usize {
                            break;
                        }

                        let sprite_pixel = (sprite_byte & (0x80 >> col)) != 0;
                        if sprite_pixel {
                            if display.draw_pixel(x_coord + col, y_coord + row, true) {
                                self.v[0xF] = 1;
                            }
                        }
                    }
                }
            }
            _ => return Err(format!("Unimplemented opcode: {:#06X}", opcode)),
        }

        Ok(())
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_initialization() {
        let cpu = CPU::new();
        assert_eq!(cpu.pc, 0x200);
        assert_eq!(cpu.v, [0; 16]);
        assert_eq!(cpu.i, 0);
        assert!(cpu.stack.is_empty());
    }

    #[test]
    fn test_clear_screen() {
        let mut cpu = CPU::new();
        let mut memory = Memory::new();
        let mut display = Display::new();

        // Set a pixel
        display.draw_pixel(0, 0, true);
        assert!(display.pixels[0][0]);

        // Execute clear screen instruction (0x00E0)
        memory.write_byte(0x200, 0x00);
        memory.write_byte(0x201, 0xE0);
        cpu.step(&mut memory, &mut display).unwrap();

        assert!(!display.pixels[0][0]);
    }
}