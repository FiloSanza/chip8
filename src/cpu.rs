#![allow(dead_code)]
extern crate rand;

use super::memory::{Data, Memory};
use super::keyboard::Keyboard;
use super::register::Register;
use super::display::Display;

use std::thread;
use rand::Rng;

const DIGITS: &[u8] = &[
    0xf0, 0x90, 0x90, 0x90, 0xf0,       //0
    0x20, 0x60, 0x20, 0x20, 0x70,       //1
    0xf0, 0x10, 0xf0, 0x80, 0xf0,       //2
    0xf0, 0x10, 0xf0, 0x10, 0xf0,       //3
    0x90, 0x90, 0xf0, 0x10, 0x10,       //4
    0xf0, 0x80, 0x90, 0x10, 0xf0,       //5
    0xf0, 0x80, 0xf0, 0x90, 0xf0,       //6
    0xf0, 0x10, 0x20, 0x40, 0x40,       //7
    0xf0, 0x90, 0xf0, 0x90, 0xf0,       //8
    0xf0, 0x90, 0xf0, 0x10, 0xf0,       //9
    0xf0, 0x90, 0xf0, 0x90, 0x90,       //A
    0xe0, 0x90, 0xe0, 0x90, 0xe0,       //B
    0xf0, 0x80, 0x80, 0x80, 0xf0,       //C
    0xe0, 0x90, 0x90, 0x90, 0xe0,       //D
    0xf0, 0x80, 0xf0, 0x80, 0xf0,       //E
    0xf0, 0x80, 0xf0, 0x80, 0x80,       //F
];

const DIGIT_INDEX: &[u16] = &[
//  0     1     2     3     4     5     6     7     8     9     A     B     C     D     E     F     
    0x00, 0x05, 0x0a, 0x0f, 0x14, 0x19, 0x1e, 0x23, 0x28, 0x2d, 0x32, 0x37, 0x3c, 0x41, 0x46, 0x4b
];

pub struct Cpu {
    pub memory: Data,
    pub display: Display,
    pub register: Register,
    pub keyboard: Keyboard,
}

impl Cpu {
    pub fn new(rom: &[u8]) -> Self {
        let mut memory = Data::new(0x1000);
        memory.data[0..DIGITS.len()].copy_from_slice(&DIGITS[..]);
        memory.data[0x200..0x200+rom.len()].copy_from_slice(&rom[..]);

        Cpu {
            memory,
            display: Display::new(),
            keyboard: Keyboard::new(),
            register: Register::new()
        }
    }

    fn get_next_u16(&mut self) -> u16 {
        let result = self.memory.get_u16(usize::from(self.register.pc));
        self.register.pc += 2;
        result
    }

    pub fn decrement_timers(&mut self) {
        if self.register.delay > 0 {
            self.register.delay -= 1;
        }
        if self.register.sound > 0 {
            self.register.sound -= 1;
        }
    }
}

impl Cpu {
    //Call subroutine at addr
    //Instructions:
    //  CALL
    fn call(&mut self, addr: u16) {
        self.register.stack.push(self.register.pc);
        self.register.pc = addr;
    }

    //Skip next Instructions if Vx = value
    //Instructions:
    //  SE  Vx, byte
    //  SE  Vx, Vy
    fn skip_if_equal(&mut self, idx: usize, value: u8) {
        if self.register.v[idx] == value {
            self.register.pc += 2;
        }
    }

    //Skip next Instructions if Vx != value
    //Instructions:
    //  SNE Vx, byte
    //  SNE Vx, Vy
    fn skip_if_not_equal(&mut self, idx: usize, value: u8) {
        if self.register.v[idx] != value {
            self.register.pc += 2;
        }
    }

    //Sets Vx = (random number[0..255] & byte)
    //Instructions:
    //  RND Vx, byte
    fn random(&mut self, idx: usize, byte: u8) {
        let mut rnd = rand::thread_rng();
        let value = rnd.gen_range(0, 256) as u8;
        self.register.v[idx] = value & byte;
    }

    //Adds two values and set the carry flag
    //Instructions:
    //  ADD Vx, Vy
    //VF = carry
    fn add(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_add(b);
        self.register.v[0xf] = ((u16::from(a) + u16::from(b)) > 0xff) as u8;
        result
    }

    //Subtract two values and set the borrow flag
    //Instructions:
    //  SUB Vx, Vy
    //  SUBN Vx, Vy
    //VF = NOT borrow
    fn sub(&mut self, a: u8, b: u8) -> u8 {
        let result = a.wrapping_sub(b);
        self.register.v[0xf] = (a > b) as u8;
        result
    }

    //Performs a left shift on a register
    //Instructions:
    //  SHL Vx
    //VF = msb(Vx) == 1
    fn shift_left(&mut self, idx: usize) {
        self.register.v[0xf] = ((self.register.v[idx] & 0x80) != 0) as u8;
        self.register.v[idx] <<= 1;
    }

    //Performs a right shift on a register
    //Instructions:
    //  SHR Vx
    //VF = lsb(Vx) == 1
    fn shift_right(&mut self, idx: usize) {
        self.register.v[0xf] = ((self.register.v[idx] & 0x01) != 0) as u8;
        self.register.v[idx] >>= 1;
    }

    //Stores registers from V0 through Vx in memory starting at index I
    //Instructions:
    //  LD  [i], Vx
    fn save_register(&mut self, end: usize) {
        for i in 0..(end+1) {
            self.memory.set_u8(usize::from(self.register.i) + i, self.register.v[i]);
        }
    }

    //Loads registers from V0 through Vx from memory starting at index I
    //Instructions:
    //  LD  Vx, [i]
    fn load_register(&mut self, end: usize) {
        for i in 0..(end+1) {
            self.register.v[i] = self.memory.get_u8(usize::from(self.register.i) + i);
        }
    }

    //Saves the BCD representation of the register Vidx in Memory[I..I+2]
    //Instructions:
    //  LD  B,  Vx
    fn save_bcd(&mut self, idx: usize) {
        let pos = self.register.i as usize;
        self.memory.set_u8(pos, self.register.v[idx] / 100);
        self.memory.set_u8(pos + 1, (self.register.v[idx] / 10) % 10);
        self.memory.set_u8(pos + 2, self.register.v[idx] % 10);
    }

    //Skip if key Vx is pressed
    //Instructions:
    //  SKP Vx
    fn skip_if_pressed(&mut self, idx: usize) {
        let key = usize::from(self.register.v[idx]);
        if self.keyboard.state[key] {
            self.register.pc += 2;
        }
    }

    //Skip if key Vx is not pressed
    //Instructions:
    //  SKNP Vx
    fn skip_if_not_pressed(&mut self, idx: usize) {
        let key = usize::from(self.register.v[idx]);
        if !self.keyboard.state[key] {
            self.register.pc += 2;
        }
    }

    //Wait for a key press, store the value in Vx
    //Instruction:
    //  LD  Vx, K
    fn wait_key(&mut self, idx: usize) {
        self.register.pc -= 2;
        for i in 0..0x10 {
            if self.keyboard.state[i] {
                self.register.v[idx] = i as u8;
                break;
            }
        }
    }
}

impl Cpu {
    pub fn next(&mut self) {
        let opcode = self.get_next_u16();
        let addr = opcode & 0x0fff;
        let nibble = opcode & 0x000f;
        let x = ((opcode & 0x0f00) >> 8) as usize;
        let y = ((opcode & 0x00f0) >> 4) as usize;
        let byte = (opcode & 0x00ff) as u8;

        match opcode >> 12 {
            0x0 => {
                match byte {
                    0xe0 => self.display.clear(),                                       //CLS
                    0xee => self.register.pc = self.register                            //RET
                                                .stack.pop()
                                                .unwrap(),
                    _ => println!("UNKNOWN {:x}", opcode),
                }
            },

            0x8 => {
                match nibble {
                    0x0 => self.register.v[x] = self.register.v[y],                     //LD    Vx, Vy
                    0x1 => self.register.v[x] |= self.register.v[y],                    //OR    Vx, Vy
                    0x2 => self.register.v[x] &= self.register.v[y],                    //AND   Vx, Vy
                    0x3 => self.register.v[x] ^= self.register.v[y],                    //XOR   Vx, Vy
                    0x4 => self.register.v[x] =                                         //ADD   Vx, Vy
                                    self.add(self.register.v[x], self.register.v[y]),
                    0x5 => self.register.v[x] =                                         //SUB   Vx, Vy
                                    self.sub(self.register.v[x], self.register.v[y]),
                    0x6 => self.shift_right(x),                                         //SHR   Vx
                    0x7 => self.register.v[x] =                                         //SUBN  Vx, Vy
                                    self.sub(self.register.v[y], self.register.v[x]),
                    0xe => self.shift_left(x),                                          //SHL   Vx
                    _ => println!("UNKNOWN {:x}", opcode),
                }
            },
            
            0xe => {
                match byte {
                    0x9e => self.skip_if_pressed(x),                                    //SKP   Vx
                    0xa1 => self.skip_if_not_pressed(x),                                //SKNP  Vx
                    _ => println!("UNKNOWN {:x}", opcode),
                }
            },

            0xf => {
                match byte {
                    0x07 => self.register.v[x] = self.register.delay,                   //LD    Vx, delay
                    0x0a => self.wait_key(x),                                           //LD    Vx, K
                    0x15 => self.register.delay = self.register.v[x],                   //LD    delay, Vx
                    0x18 => self.register.sound = self.register.v[x],                   //LD    sound, Vx
                    0x1e => {                                                           //ADD   I,  Vx
                        self.register.v[0xf] = 
                                    ((u16::from(self.register.v[x]) + self.register.i) > 0xfff) as u8;
                        self.register.i = self.register.i
                                    .wrapping_add(u16::from(self.register.v[x]));
                    },                      
                    0x29 => self.register.i =                                           //LD    F,  Vx
                                    DIGIT_INDEX[usize::from(self.register.v[x])],
                    0x33 => self.save_bcd(x),                                           //LD    V,  Vx
                    0x55 => self.save_register(x),
                    0x65 => self.load_register(x),
                    _ => println!("UNKNOWN {:x}", opcode),
                }
            }

            0x1 => self.register.pc = addr,                                             //JMP   addr
            0x2 => self.call(addr),                                                     //CALL  addr
            0x3 => self.skip_if_equal(x, byte),                                         //SE    Vx, byte
            0x4 => self.skip_if_not_equal(x as usize, byte),                            //SNE   Vx, byte
            0x5 => self.skip_if_equal(x, self.register.v[y]),                           //SE    Vx, Vy
            0x6 => self.register.v[x] = byte,                                           //LD    Vx, byte
            0x7 => self.register.v[x] = self.register.v[x]                              //ADD   Vx, byte
                                            .wrapping_add(byte),          
            0x9 => self.skip_if_not_equal(x, self.register.v[y]),                       //SNE   Vx, Vy
            0xa => self.register.i = addr,                                              //LD    I,  addr
            0xb => self.register.pc = u16::from(self.register.v[0x0])                   //JP    V0, addr
                                            .wrapping_add(addr),
            0xc => self.random(x, byte),                                                //RND   Vx, byte
            0xd => self.register.v[0xf] = self.display.draw(                            //DRW   Vx, Vy, nibble
                self.register.v[x] as usize, 
                self.register.v[y] as usize, 
                &self.memory.data[usize::from(self.register.i)..usize::from(self.register.i+nibble)]) as u8,
            _ => println!("UNKNOWN {:x}", opcode),
        }
    }
}