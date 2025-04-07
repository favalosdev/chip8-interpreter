use sdl2::keyboard::Scancode;
use std::process::Command;

pub fn beep(frequency: u32, duration_ms: u64) {
    Command::new("beep")
        .arg("-f")
        .arg(frequency.to_string())
        .arg("-l")
        .arg(duration_ms.to_string())
        .status()
        .expect("Failed to execute beep");
}

pub fn is_valid_key(code: Scancode) -> bool {
    matches!(
        code,
        Scancode::Num1
            | Scancode::Num2
            | Scancode::Num3
            | Scancode::Num4
            | Scancode::Q
            | Scancode::W
            | Scancode::E
            | Scancode::R
            | Scancode::A
            | Scancode::S
            | Scancode::D
            | Scancode::F
            | Scancode::Z
            | Scancode::X
            | Scancode::C
            | Scancode::V
    )
}
