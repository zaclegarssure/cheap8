# Cheap8
A (yet again) chip8 emulator.

## About The Project

I needed to get a bit more confident in Rust, and building a chip8 emulator (interpreter)
was a great idea. It also was a way to try different cargo features.
It uses SDL or the terminal to draw images, and ideally will support wasm one day.
It inspired by Tobias V. Langhoff's high level [tutorial](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/).
The SDL code is really similar to the [examples](https://docs.rs/sdl2/0.34.5/sdl2/) provided in the crate.

## Getting Started

You should download a chip8 ROM on the internet, [here](https://github.com/kripod/chip8-roms) for instance.
Then for the SDL version:
```
cargo run -p cheap8_sdl <path-to-rom>
```
With the terminal based:
```
cargo run -p cheap8_terminal <path-to-rom>
```
### Prerequisites

You need to have SDL2 installed on your system to use the SDL client,
instructions can be found [here](https://github.com/Rust-SDL2/rust-sdl2).

## Contributing

Why would you contribute to such a project ?

In anycase contribution will always be appreciated.
