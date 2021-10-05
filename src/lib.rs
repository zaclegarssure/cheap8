//! # Cheap8
//!
//! `Cheap8` is a simple Chip8 emulator, bundled with
//! mutliple clients, like a sdl based, terminal based 
//! adn hopefully wasm one.
mod cpu;
mod display;
mod timer;

pub use cpu::{Cpu, Output};
pub use display::{Display, HEIGHT, WIDTH};
