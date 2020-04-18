use chip8::disassemble;
use std::fs::File;
use std::io::Read;

fn run(buffer: &Vec<u8>) {
    let mut pc = 0x200;

    while pc < buffer.len() {
        let opcode: u16 = (u16::from(buffer[pc]) << 8) | u16::from(buffer[pc+1]);
        disassemble(opcode);
        pc += 2;
    }
}

pub fn main() {
    let mut file = File::open("./rom/IBM").unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    let mut memory: Vec<u8> = vec![0x0; 0x200+buffer.len()];
    memory[0x200..].copy_from_slice(&buffer[..]);
    run(&memory);
}