use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread;
use std::env;

mod drivers;
use drivers::DisplayDriver;

use cheap8::{Cpu,Output};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context);
    
    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load(&args[1]);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let Output { screen, screen_update, beep: _ } = cpu.cycle(&[false;16]);
        if screen_update {
            display_driver.draw(screen);
        }

        // The rest of the game loop goes here...

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
}
