use clap::Parser;
use sdl2;

use std::thread;
use std::time::Duration;

mod drivers;
use drivers::AudioDriver;
use drivers::DisplayDriver;
use drivers::InputDriver;

use cheap8_core::{Cpu, Output};

mod parse_args;
use parse_args::Cli;

pub fn main() {
    let args = Cli::parse();
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context, &args)
        .expect("Failed to create a display driver");
    let mut input_driver = InputDriver::new(&sdl_context)
        .expect("Failed to create an input driver");
    let audio_driver = AudioDriver::new(&sdl_context)
        .expect("Failed to create an audio driver");

    let mut cpu = Cpu::new();
    cpu.reset();
    cpu.load(args.path.to_str().unwrap());

    while let Some(inputs) = input_driver.poll() {
        let Output {
            screen,
            screen_update,
            beep,
        } = cpu.cycle(&inputs);
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
