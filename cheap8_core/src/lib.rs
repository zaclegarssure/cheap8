//! # cheap8_core
//! This is the core crate of cheap8, it
//! contains all strucs and implementation to
//! have a chip8 interpreter (internally).
mod cpu;
mod display;
mod timer;

pub use cpu::{Cpu, Output};
pub use display::{Display, HEIGHT, WIDTH};
