#![allow(dead_code)]
extern crate minifb;

use minifb::Key;

pub const BINDINGS: &[(Key, usize)]  = &[
    (Key::Key1, 0x1),
    (Key::Key2, 0x2),
    (Key::Key3, 0x3),
    (Key::Key4, 0xc),
    (Key::Q, 0x4),
    (Key::W, 0x5),
    (Key::E, 0x6),
    (Key::R, 0xd),
    (Key::A, 0x7),
    (Key::S, 0x8),
    (Key::D, 0x9),
    (Key::F, 0xe),
    (Key::Z, 0xa),
    (Key::X, 0x0),
    (Key::C, 0xb),
    (Key::V, 0xf),
];

pub struct Keyboard {
    pub state: Vec<bool>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            state: vec![false; 0x10],
        }
    }

    pub fn set_key(&mut self, idx: usize, value: bool) {
        self.state[idx] = value
    }
}