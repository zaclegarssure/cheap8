use sdl2;

use std::time::Duration;
use std::thread;
use std::env;

mod drivers;
use drivers::DisplayDriver;
use drivers::InputDriver;
use drivers::AudioDriver;

use cheap8::{Cpu,Output};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context);
    let mut input_driver = InputDriver::new(&sdl_context);
    let audio_driver = AudioDriver::new(&sdl_context);

    let args: Vec<String> = env::args().collect();
    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load(&args[1]);

    while let Some(inputs) = input_driver.poll() {

        let Output { screen, screen_update, beep } = cpu.cycle(&inputs);
        if screen_update {
            display_driver.draw(screen);
        }

        if beep {
            audio_driver.play();
        } else {
            audio_driver.stop();
        }


        thread::sleep(Duration::from_millis(1));
    }

}
