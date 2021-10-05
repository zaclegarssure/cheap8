use sdl2;
use structopt::StructOpt;

use std::thread;
use std::time::Duration;

mod drivers;
use drivers::AudioDriver;
use drivers::DisplayDriver;
use drivers::InputDriver;

use cheap8::{Cpu, Output};

mod parse_args;
use parse_args::Cli;

pub fn main() {
    let args = Cli::from_args();
    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context, &args);
    let mut input_driver = InputDriver::new(&sdl_context);
    let audio_driver = AudioDriver::new(&sdl_context);

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
