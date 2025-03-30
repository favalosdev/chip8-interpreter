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
