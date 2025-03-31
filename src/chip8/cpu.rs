use super::{constants::*, display::Display, keyboard::Keyboard, memory::Memory, utils::beep};
use rand::prelude::*;
use std::thread;

pub struct CPU {
    // Program counter
    pc: u16,
    // General purpose registers V0-VF
    v: [u8; 16],
    // Index register
    i: u16,
    // Stack for subroutines
    stack: Vec<u16>,
    sound_timer: u8,
    delay_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x200, // Program starts at 0x200
            v: [0; 16],
            i: 0,
            stack: Vec::new(),
            sound_timer: 0,
            delay_timer: 0,
        }
    }

    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut Display,
        keyboard: &Keyboard,
    ) -> Result<(), String> {
        let opcode = self.fetch(memory);
        return self.execute(opcode, memory, display, keyboard);
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
        keyboard: &Keyboard,
    ) -> Result<(), String> {
        // Decode opcode parts
        let opcode_class = (opcode & 0xF000) >> 12;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn: u16 = opcode & 0x0FFF;

        // Increment PC by default (some instructions will override this)
        self.pc += 2;

        let error: Result<(), String> = Err(format!("Unknown opcode: {:#06X}", opcode));
        let mut rng = rand::rng();

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
                self.pc = nnn;
            }
            0x2 => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }
            0x3 => {
                if self.v[x] == nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                if self.v[x] != nn {
                    self.pc += 2;
                }
            }
            0x5 => match n {
                0 => {
                    if self.v[x] == self.v[y] {
                        self.pc += 2;
                    }
                }
                _ => return error,
            },
            0x6 => {
                self.v[x] = nn;
            }
            0x7 => {
                self.v[x] += nn;
            }
            0x8 => match n {
                0 => {
                    self.v[x] = self.v[y];
                }
                1 => {
                    self.v[x] = self.v[x] | self.v[y];
                }
                2 => {
                    self.v[x] = self.v[x] & self.v[y];
                }
                3 => {
                    self.v[x] = self.v[x] ^ self.v[y];
                }
                4 => {
                    let result = (self.v[x] + self.v[y]) as u16;
                    self.v[0xF] = u8::from(result > 255);
                    self.v[x] = (result & 0xFF) as u8;
                }
                5 => {
                    let flag = self.v[x] > self.v[y];
                    self.v[0xF] = u8::from(flag);
                    self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                }
                6 => {
                    let bit = self.v[x] & 0b00000001;
                    self.v[0xF] = u8::from(bit);
                    self.v[x] >>= 1;
                }
                7 => {
                    let flag = self.v[y] > self.v[x];
                    self.v[0xF] = u8::from(flag);
                    self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                }
                0xE => {
                    let bit = self.v[x] & 0b10000000;
                    self.v[0xF] = u8::from(bit);
                    self.v[x] = self.v[x].wrapping_mul(2);
                }
                _ => return error,
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.i = nnn;
            }
            0xB => {
                self.pc = nnn + (self.v[0] as u16);
            }
            0xC => {
                let random = rng.random::<u8>();
                self.v[x] = random & nn;
            }
            0xD => {
                // Display/draw
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
                    let key = keyboard.get_pressed_key() as u8;
                    if self.v[x] == key {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    let key = keyboard.get_pressed_key() as u8;
                    if self.v[x] != key {
                        self.pc += 2;
                    }
                }
                _ => return error,
            },
            0xF => match nn {
                0x07 => {
                    self.v[x] = self.delay_timer;
                }
                0x0A => {
                    // FROG: all execution stops until a key is pressed
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
                    // FROG: location of sprite for digit Vx
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
                    for (i, register) in self.v.iter().enumerate() {
                        memory.write_byte(start + i, *register);
                    }
                }
                0x65 => {
                    let start = self.i as usize;
                    for (i, register) in self.v.iter_mut().enumerate() {
                        *register = memory.read_byte(start + i);
                    }
                }
                _ => return error,
            },
            _ => return error,
        }
        Ok(())
    }
}
