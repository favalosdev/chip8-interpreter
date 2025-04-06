use chip8::constants::{SDL_FREQUENCY, VALUE_KEY_MAP};
use chip8::utils::KeyboardState;
use sdl2::{event::Event, keyboard::Keycode, keyboard::Scancode};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

use chip8::{
    constants::{
        CPU_FREQUENCY, ORIGINAL_HEIGHT, ORIGINAL_WIDTH, SCALE_FACTOR, TIMER_DECREASE_FREQUENCY,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    cpu::CPU,
    display::Display,
    memory::Memory,
};

mod chip8;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1).ok_or("Usage: chip8-emulator <rom_file>")?;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut keyboard_state: KeyboardState = KeyboardState {
        last_key_pressed: None,
        last_hex: None,
        waiting_for_key: false,
    };

    // Initialize CHIP-8 components
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut display = Display::new();

    let mut rom_file = File::open(rom_path).map_err(|e| e.to_string())?;
    let mut rom_data = Vec::new();

    rom_file
        .read_to_end(&mut rom_data)
        .map_err(|e| e.to_string())?;

    memory.load_rom(&rom_data);

    let mut last_timers_tick = Instant::now();
    let mut last_cpu_tick = Instant::now();
    let mut last_sdl_tick = Instant::now();

    let timers_interval = Duration::from_nanos(1_000_000_000 / TIMER_DECREASE_FREQUENCY);
    let cpu_interval = Duration::from_nanos(1_000_000_000 / CPU_FREQUENCY);
    let sdl_interval = Duration::from_nanos(1_000_000_000 / SDL_FREQUENCY);

    let scan_to_hex: HashMap<Scancode, u8> = {
        let mut map = HashMap::new();
        for &(hex, scan) in &VALUE_KEY_MAP {
            map.insert(scan, hex);
        }
        map
    };

    'main: loop {
        let now = Instant::now();

        if now.duration_since(last_timers_tick) >= timers_interval {
            cpu.update_timers();
            last_timers_tick = now;
        }

        if now.duration_since(last_cpu_tick) >= cpu_interval {
            if let Err(e) = cpu.step(&mut memory, &mut display, &mut keyboard_state) {
                eprintln!("CPU error: {}", e);
                break 'main;
            }

            last_cpu_tick = now;
        }

        if now.duration_since(last_sdl_tick) >= sdl_interval {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    Event::KeyDown {
                        scancode: Some(code),
                        ..
                    } => {
                        keyboard_state.last_key_pressed = Some(code);
                        keyboard_state.last_hex = scan_to_hex.get(&code).map(|x| *x);

                        if keyboard_state.waiting_for_key {
                            keyboard_state.waiting_for_key = false;
                        }
                    }
                    _ => {}
                }
            }

            if display.changed {
                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                canvas.clear();

                canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));

                for y in 0..ORIGINAL_HEIGHT as usize {
                    for x in 0..ORIGINAL_WIDTH as usize {
                        if display.pixels[y][x] {
                            let rect = sdl2::rect::Rect::new(
                                (x as u32 * SCALE_FACTOR) as i32,
                                (y as u32 * SCALE_FACTOR) as i32,
                                SCALE_FACTOR,
                                SCALE_FACTOR,
                            );
                            canvas.fill_rect(rect)?;
                        }
                    }
                }

                canvas.present();
                display.changed = false;
            }

            last_sdl_tick = now;
        }
    }

    Ok(())
}
