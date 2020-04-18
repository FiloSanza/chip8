pub trait Memory {
    fn get_u16(&self, idx: usize) -> u16;
    fn set_u8(&mut self, idx: usize, value: u8);
    fn get_u8(&self, idx: usize) -> u8;
    fn clear(&mut self);
}

#[derive(Default)]
pub struct Data {
    pub data: Vec<u8>,
}

impl Data {
    pub fn new(size: usize) -> Self {
        Data {
            data: vec![0x0; size]
        }
    }
}

impl Memory for Data {
    fn get_u16(&self, idx: usize) -> u16 {
        u16::from(self.data[idx]) << 8 | u16::from(self.data[idx+1])
    }

    fn set_u8(&mut self, idx: usize, value: u8) {
        self.data[idx] = value;
    }
    
    fn get_u8(&self, idx: usize) -> u8 {
        self.data[idx]
    }

    fn clear(&mut self) {
        for i in self.data.iter_mut() {
            *i = 0;
        }
    }
}