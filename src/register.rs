#[derive(Default, Debug)]
pub struct Register {
    pub v: Vec<u8>,         //16 8-bit register indexed from 0x0 to 0xF, V[0xF] contains Flags
    pub i: u16,             //only the first 12 bits are used, memory address
    pub pc: u16,            //Program Counter
    pub stack: Vec<u16>,    //Stack
    pub sound: u8,          //Sound timer
    pub delay: u8,          //Delay timer
}

impl Register {
    pub fn new() -> Self {
        Register {
            v: vec![0x0; 0x10],
            i: 0,
            pc: 0x200,
            stack: Vec::new(),
            sound: 0,
            delay: 0,
        }
    }
}