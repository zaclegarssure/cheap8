use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::prelude::*;

use crate::display::{Display,WIDTH,HEIGHT};
use crate::timer::Timer;

const START_PC: u16 = 0x200;
const FONT: [u8;80] =
    [0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80];  // F

pub struct Output<'a> {
    pub screen: &'a[bool;WIDTH*HEIGHT],
    pub screen_update: bool,
    pub beep: bool,
}

pub struct Cpu {
    register: [u8;16],
    index: u16,
    pc: u16,
    stack: [u16;16],
    sp: u8,
    delay_timer: Timer,
    sound_timer: Timer,
    memory: [u8;4096],
    display: Display,
    rng: ThreadRng,
    update_screen: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: [0;16],
            index: 0,
            pc: 0,
            stack: [0;16],
            sp: 0,
            delay_timer: Timer::new(),
            sound_timer: Timer::new(),
            memory: [0;4096],
            display: Display::new(),
            rng: rand::thread_rng(),
            update_screen: false,
        }
    }

    pub fn reset(&mut self) -> () {
        self.register = [0;16];
        self.index = 0;
        self.pc = START_PC;
        self.stack = [0;16];
        self.sp = 0;
        self.delay_timer.reset();
        self.sound_timer.reset();
        self.memory = [0;4096];
        for i in 0..FONT.len() {
            self.memory[i] = FONT[i];
        }
        self.display.clear();
        self.rng = rand::thread_rng();
    }

    pub fn cycle(&mut self, key_pressed: &[bool;16]) -> Output {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        self.delay_timer.decrement();
        let beep = self.sound_timer.decrement();
        self.update_screen = false;
        self.execute(opcode, key_pressed);

        Output {
            screen: self.display.get(),
            screen_update: self.update_screen,
            beep,
        }
    }

    fn execute(&mut self, opcode: u16, key_pressed: &[bool;16]) -> () {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;
        let vx = self.register[x];
        let vy = self.register[y];

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;


        match op_1 {
            0x0 => match (op_3,op_4) {
                (0xE,0x0) => {
                    self.display.clear();
                    self.update_screen = true;
                } //clear screen
                (0xE,0xE) => self.pc = self.pop(), //return
                _ => panic!("Unsupported instruction {:#x}{:#x}{:#x}{:#x}",op_1,op_2,op_3,op_4),
            }
            0x1 => self.pc = nnn, //jump
            0x2 => {
                self.push(self.pc);
                self.pc = nnn;
            },
            0x3 => if vx == nn {
                self.pc += 2;
            },
            0x4 => if vx != nn {
                self.pc += 2;
            },
            0x5 => if vx == vy {
                self.pc += 2;
            },
            0x9 => if vx != vy {
                self.pc += 2;
            },
            0x6 => self.register[x] = nn, //set register
            0x7 => self.register[x] = vx.wrapping_add(nn), //add to register
            0x8 => match op_4 {
                0x0 => self.register[x] = vy,
                0x1 => self.register[x] = vx | vy,
                0x2 => self.register[x] = vx & vy,
                0x3 => self.register[x] = vx ^ vy,
                0x4 => {
                    let (res,ovf) = vx.overflowing_add(vy);
                    self.register[x] = res;
                    self.register[0xF] = ovf as u8;
                }
                0x5 => {
                    self.register[x] = vx.wrapping_sub(vy);
                    self.register[0xF] = (vx >= vy) as u8;
                }
                0x6 => {
                    self.register[0xF] = vx & 0x1;
                    self.register[x] >>= 1;
                }
                0x7 => {
                    self.register[x] = vy.wrapping_sub(vx);
                    self.register[0xF] = (vy >= vx) as u8;
                }
                0xE => {
                    self.register[0xF] = vx & 0x80;
                    self.register[x] <<= 1;
                },
                _ => panic!("Unsupported instruction {:#x}{:#x}{:#x}{:#x}",op_1,op_2,op_3,op_4),
            },
            0xA => self.index = nnn,
            0xB => self.pc = nnn + self.register[0] as u16,
            0xC => {
                let rnd: u8 = self.rng.gen();
                self.register[x] = rnd & nn;
            },
            0xE => match (op_3,op_4) {
                (0x9,0xE) => if key_pressed[vx as usize] { self.pc += 2; },
                (0xA,0x1) => if !key_pressed[vx as usize] { self.pc += 2; },
                _ => panic!("Unsupported instruction {:#x}{:#x}{:#x}{:#x}",op_1,op_2,op_3,op_4),
            }
            0xF => match (op_3, op_4) {
                (0x0,0xA) => {
                    for (i, key) in key_pressed.into_iter().enumerate() {
                        if *key {
                            self.register[x] = i as u8;
                            return;
                        }
                    }
                    self.pc -= 2;
                }
                (0x0,0x7) => self.register[x] = self.delay_timer.timer,
                (0x1,0x5) => self.delay_timer.timer = vx,
                (0x1,0x8) => self.sound_timer.timer = vx,
                (0x1,0xE) => self.index += vx as u16,
                (0x2,0x9) => self.index = (vx & 0xF) as u16 * 5,
                (0x3,0x3) => {
                    let digit1 = vx / 100;
                    let digit2 = (vx % 100) / 10;
                    let digit3 = vx % 10;
                    self.memory[self.index as usize] = digit1;
                    self.memory[(self.index+1) as usize] = digit2;
                    self.memory[(self.index+2) as usize] = digit3;
                }
                (0x5,0x5) => {
                    for i in 0..=x {
                        self.memory[self.index as usize + i] = self.register[i];
                    }
                }
                (0x6,0x5) => {
                    for i in 0..=x {
                        self.register[i] = self.memory[self.index as usize + i];
                    }
                }
                _ => panic!("Unsupported instruction {:#x}{:#x}{:#x}{:#x}",op_1,op_2,op_3,op_4),
            }
            0xD => {
                let vf = self.display.draw(vx as usize, vy as usize, &self.memory[self.index as usize..(self.index + n) as usize]);
                self.register[0xF] = vf as u8;
                self.update_screen = true;
            }
            _ => panic!("Unsupported instruction {:#x}{:#x}{:#x}{:#x}",op_1,op_2,op_3,op_4),
        }
    }

    fn push(&mut self, value: u16) -> () {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn load(&mut self, path: &str) {
        let path = Path::new(path);

        let mut file = match File::open(path) {
            Err(_) => panic!("Lmao"),
            Ok(file) => file,
        };

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer);

        for i in 0x200..self.memory.len().min(buffer.len() + 0x200) {
            self.memory[i] = buffer.get(i - 0x200).unwrap().clone();
        }

    }
}
