use sdl2;

use std::time::Duration;
use std::thread;
use std::env;

mod drivers;
use drivers::DisplayDriver;
use drivers::InputDriver;

use cheap8::{Cpu,Output};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);

    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load(&args[1]);

    'running: loop {

        if let Some(inputs) = input_driver.poll() {
            let Output { screen, screen_update, beep: _ } = cpu.cycle(&inputs);
            if screen_update {
                display_driver.draw(screen);
            }

        } else {
            break 'running;
        }


        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
