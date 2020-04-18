#![allow(dead_code)]

use super::bit;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const WHITE: u32 = 0xffffff;
const BLACK: u32 = 0x000000;

pub struct Display {
    memory: Box<Vec<bool>>,
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory: Box::new(vec![false; WIDTH * HEIGHT])
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool{
        self.memory[y * WIDTH + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.memory[y * WIDTH + x] = value
    }

    pub fn clear(&mut self) {
        for i in 0..WIDTH*HEIGHT {
            self.memory[i] = false;
        }
    }
    
    //Draw a sprite of n bytes, if there is a collision VF is set to 1
    //Instructions:
    //  DRW Vx, Vy, nibble
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for i in 0..sprite.len() {
            let byte = sprite[i];
            for j in 0..8 {
                let value = bit::get(byte, 7 - j);
                if value {
                    let posx = (j + x) % WIDTH;
                    let posy = (i + y) % HEIGHT;
                    let old_value = self.get_pixel(posx, posy);
                    collision |= old_value & value;
                    self.set_pixel(posx, posy, value ^ old_value);
                }
            }
        }

        collision
    }

    pub fn get_buffer(&self) -> Vec<u32> {
        let mut buffer = vec![BLACK; self.memory.len()];

        for (i, value) in self.memory.iter().enumerate() {
            buffer[i] = if *value { WHITE } else { BLACK };
        }

        buffer
    }
}