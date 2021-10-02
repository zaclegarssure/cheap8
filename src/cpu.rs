use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::display::Display;

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

pub struct Cpu {
    pub register: [u8;16],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16;16],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub memory: [u8;4096],
    pub display: Display,

}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            register: [0;16],
            index: 0,
            pc: 0,
            stack: [0;16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: [0;4096],
            display: Display::new(),
        }
    }

    pub fn reset(&mut self) -> () {
        self.register = [0;16];
        self.index = 0;
        self.pc = START_PC;
        self.stack = [0;16];
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.memory = [0;4096];
        for i in 0..FONT.len() {
            self.memory[i] = FONT[i];
        }
        self.display.clear();
    }

    pub fn cycle(&mut self) -> () {
        let opcode: u16 = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u16) -> () {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1,op_2,op_3,op_4) {
            (0x0,_,_,_) => match (op_3,op_4) {
                (0xE,0x0) => self.display.clear(), //clear screen
                (0xE,0xE) => self.pc = self.pop(), //return
                _ => panic!("Unsupported instruction"), //SYS
            }
            (0x1,_,_,_) => self.pc = nnn, //jump
            (0x2,_,_,_) => {
                self.push(self.pc);
                self.pc = nnn;
            },
            (0x3,0,0,0) => (),
            (0x4,0,0,0) => (),
            (0x5,0,0,0) => (),
            (0x6,_,_,_) => self.register[x] = nn, //set register
            (0x7,_,_,_) => self.register[x] += nn, //add to register
            (0x8,0,0,0) => (),
            (0xA,_,_,_) => self.index = nnn,
            (0xD,_,_,_) => { 
                let vf = self.display.draw(self.register[x] as usize, self.register[y] as usize, &self.memory[self.index as usize..(self.index + n) as usize]); 
                self.register[0xF as usize] = vf as u8;
            },
            _ => panic!("Unsupported instruction"),
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
