extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    
    let video_subsystem = sdl_context.video().unwrap();
    let _window = video_subsystem.window("SDL2 with Rust", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    println!("Press Enter to quit...");
    let _ = std::io::stdin().read_line(&mut String::new());
}
