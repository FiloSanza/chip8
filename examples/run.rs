use minifb::{Window, WindowOptions, Scale};
use chip8::{Cpu, BINDINGS};
use std::io::{Read, stdin};
use std::fs::File;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

const ROMS: &[&'static str] = &[
    "./rom/IBM",
    "./rom/INVADERS",
    "./rom/MAZE",
    "./rom/MISSILE",
];

fn rom_number() -> usize {
    let mut rom = String::new();

    for (idx, name) in ROMS.iter().enumerate() {
        println!("{}:\t{}", idx, name);
    }

    println!("Enter the rom number");
    stdin().read_line(&mut rom)
        .expect("Error reading rom number");

    rom.trim().parse::<usize>().unwrap()
}

pub fn run(rom: &[u8], name: &str) {
    let mut cpu = Cpu::new(rom);
    let mut window = Window::new(
        name,
        WIDTH,
        HEIGHT,
        WindowOptions{
            scale: Scale::X8,
            ..WindowOptions::default()
        }
    ).unwrap();

    while window.is_open() {
        for i in 0..10 {
            cpu.next();
        }

        cpu.decrement_timers();

        for (key, idx) in BINDINGS {
            cpu.keyboard.set_key(*idx, window.is_key_down(*key));
        }

        window.update_with_buffer(&cpu.display.get_buffer(), WIDTH, HEIGHT).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

pub fn main() {
    let idx = rom_number();
    let mut file = File::open(ROMS[idx]).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    run(&buffer, ROMS[idx]);
}