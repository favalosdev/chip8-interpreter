extern crate fps_clock;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::thread;

use chip8::{
    constants::{
        ORIGINAL_HEIGHT, ORIGINAL_WIDTH, SCALE_FACTOR, TIMER_DECREASE_RATE, WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
    cpu::CPU,
    display::Display,
    keyboard::Keyboard,
    memory::Memory,
};

mod chip8;

fn beep(frequency: u32, duration_ms: u64) {
    Command::new("beep")
        .arg("-f")
        .arg(frequency.to_string())
        .arg("-l")
        .arg(duration_ms.to_string())
        .status()
        .expect("Failed to execute beep");
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let rom_path = args.get(1).ok_or("Usage: chip8-emulator <rom_file>")?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    // Initialize CHIP-8 components
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    let mut display = Display::new();
    let mut keyboard = Keyboard::new();

    let mut rom_file = File::open(rom_path).map_err(|e| e.to_string())?;
    let mut rom_data = Vec::new();

    rom_file
        .read_to_end(&mut rom_data)
        .map_err(|e| e.to_string())?;

    memory.load_rom(&rom_data);

    let mut sound_timer: u8 = 0;
    let mut sound_fps = fps_clock::FpsClock::new(TIMER_DECREASE_RATE);

    'sound_loop: loop {
        if sound_timer > 0 {
            sound_timer = sound_timer.saturating_sub(1);
            thread::spawn(|| {
                beep(440, 500);
            });
        } else {
            break 'sound_loop;
        }
        sound_fps.tick();
    }

    let mut delay_timer: u8 = 0;
    let mut delay_fps = fps_clock::FpsClock::new(TIMER_DECREASE_RATE);

    'delay_loop: loop {
        if delay_timer > 0 {
            delay_timer = delay_timer.saturating_sub(1);
        } else {
            break 'delay_loop;
        }

        delay_fps.tick();
    }

    'sdl_running: loop {
        // Handle SDL events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'sdl_running,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => keyboard.press_key(scancode),
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => keyboard.release_key(scancode),
                _ => {}
            }
        }

        // Execute CPU cycle
        if let Err(e) = cpu.step(&mut memory, &mut display) {
            eprintln!("CPU error: {}", e);
            break 'sdl_running;
        }

        // Update timers
        // cpu.update_timers();

        // Render display
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
    }

    Ok(())
}
