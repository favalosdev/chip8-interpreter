use super::{constants::*, display::Display, memory::Memory, utils::beep, utils::KeyboardState};
use rand::prelude::*;
use std::thread;

pub struct CPU {
    pc: u16,
    v: [u8; 16],
    i: u16,
    stack: Vec<u16>,
    sound_timer: u8,
    delay_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: PROGRAM_START_ADDRESS,
            v: [0; 16],
            i: 0,
            stack: Vec::new(),
            sound_timer: 0,
            delay_timer: 0,
        }
    }

    pub fn update_timers(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer = self.sound_timer.wrapping_sub(1);
            thread::spawn(move || {
                beep(440, 550);
            });
        }

        if self.delay_timer > 0 {
            self.delay_timer = self.delay_timer.wrapping_sub(1);
        }
    }

    fn advance_pc(&mut self) {
        self.pc = self.pc.wrapping_add(2);
    }

    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keyboard_state: &mut KeyboardState,
    ) -> Result<(), String> {
        let opcode = self.fetch(memory);
        self.execute(opcode, memory, display, keyboard_state)
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
        keyboard_state: &mut KeyboardState,
    ) -> Result<(), String> {
        // Decode opcode parts
        let opcode_class = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn: u16 = opcode & 0x0FFF;

        let error: Result<(), String> = Err(format!("Unknown opcode: {:#06X}", opcode));
        let mut rng = rand::rng();

        self.advance_pc();

        match opcode_class {
            0x0 => {
                match opcode {
                    0x00E0 => display.clear(),
                    0x00EE => {
                        // Return from subroutine
                        self.pc = self.stack.pop().ok_or("Stack underflow")?;
                    }
                    _ => return error,
                }
            }
            0x1 => {
                // Overrides increment
                self.pc = nnn;
            }
            0x2 => {
                self.stack.push(self.pc);
                // Overrides increment
                self.pc = nnn;
            }
            // Rethink skip instructions
            0x3 => {
                if self.v[x] == nn {
                    self.advance_pc();
                }
            }
            0x4 => {
                if self.v[x] != nn {
                    self.advance_pc();
                }
            }
            0x5 => match n {
                0 => {
                    if self.v[x] == self.v[y] {
                        self.advance_pc();
                    }
                }
                _ => return error,
            },
            0x6 => {
                self.v[x] = nn;
            }
            0x7 => {
                self.v[x] = self.v[x].wrapping_add(nn);
            }
            0x8 => match n {
                0x0 => {
                    self.v[x] = self.v[y];
                }
                0x1 => {
                    self.v[x] |= self.v[y];
                }
                0x2 => {
                    self.v[x] &= self.v[y];
                }
                0x3 => {
                    self.v[x] ^= self.v[y];
                }
                0x4 => {
                    let (result, overflow) = self.v[x].overflowing_add(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(overflow);
                }
                0x5 => {
                    let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(!borrow);
                }
                0x6 => {
                    let prev = self.v[x];
                    self.v[x] >>= 1;
                    self.v[0xF] = prev & 1;
                }
                0x7 => {
                    let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                    self.v[x] = result;
                    self.v[0xF] = u8::from(!borrow);
                }
                0xE => {
                    let prev = self.v[x];
                    self.v[x] <<= 1;
                    self.v[0xF] = (prev >> 7) & 1;
                }
                _ => return error,
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.advance_pc();
                }
            }
            0xA => {
                self.i = nnn;
            }
            0xB => {
                // Overrides increment
                self.pc = nnn.wrapping_add(self.v[0] as u16);
            }
            0xC => {
                let random = rng.random::<u8>();
                self.v[x] = random & nn;
            }
            0xD => {
                self.v[0xF] = 0;
                let x_coord = self.v[x] as usize % ORIGINAL_WIDTH as usize;
                let y_coord = self.v[y] as usize % ORIGINAL_HEIGHT as usize;
                let height = n as usize;

                for row in 0..height {
                    if y_coord + row < ORIGINAL_HEIGHT as usize {
                        let sprite_byte = memory.read_byte(self.i as usize + row);
                        for col in 0..SPRITE_WIDTH {
                            if x_coord + col < ORIGINAL_WIDTH as usize {
                                let to_draw = (sprite_byte & (0x80 >> col)) != 0;
                                if display.draw_pixel(x_coord + col, y_coord + row, to_draw) {
                                    self.v[0xF] = 1;
                                }
                            }
                        }
                    }
                }
            }
            0xE => match nn {
                0x9E => {
                    if let Some(hex) = keyboard_state.last_hex {
                        if hex == self.v[x] {
                            self.advance_pc();
                        }
                    }
                }
                0xA1 => {
                    if let Some(hex) = keyboard_state.last_hex {
                        if hex != self.v[x] {
                            self.advance_pc();
                        }
                    }
                }
                _ => return error,
            },
            0xF => match nn {
                0x07 => {
                    self.v[x] = self.delay_timer;
                }
                0x0A => {
                    keyboard_state.waiting_for_key = true;

                    // This busy wait is necessary because it will be updated in the main thread
                    while keyboard_state.waiting_for_key {}

                    if let Some(hex) = keyboard_state.last_hex {
                        self.v[x] = hex;
                    }
                }
                0x15 => {
                    self.delay_timer = self.v[x];
                }
                0x18 => {
                    self.sound_timer = self.v[x];
                }
                0x1E => {
                    self.i = self.i.wrapping_add(self.v[x] as u16);
                }
                0x29 => {
                    let hex = (self.v[x] & 0x0F) as u16;
                    self.i = (FONT_DATA_START_ADDRESS as u16) + 5 * hex;
                }
                0x33 => {
                    let hundreds = self.v[x] / 100;
                    let tens = (self.v[x] / 10) % 10;
                    let ones = self.v[x] % 10;

                    memory.write_byte(self.i as usize, hundreds);
                    memory.write_byte((self.i + 1) as usize, tens);
                    memory.write_byte((self.i + 2) as usize, ones);
                }
                0x55 => {
                    let start = self.i as usize;
                    for j in 0..=x {
                        memory.write_byte(start + j, self.v[j]);
                    }
                }
                0x65 => {
                    let start = self.i as usize;
                    for j in 0..=x {
                        self.v[j] = memory.read_byte(start + j);
                    }
                }
                _ => return error,
            },
            _ => return error,
        }
        Ok(())
    }
}
