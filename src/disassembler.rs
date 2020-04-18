pub fn disassemble(opcode: u16) {
    print!("{:x}\t", opcode);
    let x = ((opcode & 0x0f00) >> 8) as usize;
    let y = ((opcode & 0x00f0) >> 4) as usize;
    let addr = opcode & 0x0fff;
    let byte = (opcode & 0x00ff) as u8;
    let nibble = opcode & 0x000f;

    match opcode >> 12 {
        0x0 => {
            match byte {
                0xe0 => println!("CLS"),                                       //CLS
                0xee => println!("RET"),
                _ => println!("UNKNOWN {:x}", opcode),
            }
        },

        0x8 => {
            match nibble {
                0x0 => println!("LD\tV{}, V{}", x, y),                     //LD    Vx, Vy
                0x1 => println!("OR\tV{}, V{}", x, y),                    //OR    Vx, Vy
                0x2 => println!("AND\tV{}, V{}", x, y),                    //AND   Vx, Vy
                0x3 => println!("XOR\tV{}, V{}", x, y),                    //XOR   Vx, Vy
                0x4 => println!("ADD\tV{}, V{}", x, y),
                0x5 => println!("SUB\tV{}, V{}", x, y),
                0x6 => println!("SHR\tV{}", x),                                         //SHR   Vx
                0x7 => println!("SUBN\tV{}, V{}", x, y),
                0xe => println!("SHR\tV{}", x),                                          //SHL   Vx
                _ => println!("UNKNOWN {:x}", opcode),
            }
        },
        
        0xe => {
            match byte {
                0x9e => println!("SKP\tV{}", x),                                    //SKP   Vx
                0xa1 => println!("SKNP\tV{}", x),                                //SKNP  Vx
                _ => println!("UNKNOWN {:x}", opcode),
            }
        },

        0xf => {
            match byte {
                0x07 => println!("LD\tV{}, delay", x),                   //LD    Vx, delay
                0x0a => println!("LD\tV{}, K", x),                                           //LD    Vx, K
                0x15 => println!("LD\tdelay, V{}", x),                   //LD    delay, Vx
                0x18 => println!("LD\tsound, V{}", x),                   //LD    sound, Vx
                0x1e => println!("ADD\tI, V{}", x),
                0x29 => println!("LD\tF, V{}", x),
                0x33 => println!("LD\tB, V{}", x),                                           //LD    B,  Vx
                0x55 => println!("LD\t[I], V{}", x),
                0x65 => println!("LD\tV{}, [I]", x),
                _ => println!("UNKNOWN {:x}", opcode),
            }
        }

        0x1 => println!("JMP\t{}", addr),                                             //JMP   addr
        0x2 => println!("CALL\t{}", addr),                                                     //CALL  addr
        0x3 => println!("SE\tV{}, {}", x, byte),                                         //SE    Vx, byte
        0x4 => println!("SNE\tV{}, {}", x, byte),                            //SNE   Vx, byte
        0x5 => println!("SE\tV{}, V{}", x, y),                           //SE    Vx, Vy
        0x6 => println!("LD\tV{}, {}", x, byte),                                           //LD    Vx, byte
        0x7 => println!("ADD\tV{}, {}", x, byte),          
        0x9 => println!("SNE\tV{}, V{}", x, y),                           //SNE   Vx, Vy
        0xa => println!("LD\tI, {}", addr),                                              //LD    I,  addr
        0xb => println!("JP\tV0, {}", addr),
        0xc => println!("RND\tV{}, {}", x, byte),                                                //RND   Vx, byte
        0xd => println!("DRW\t{}, {}, {}", x, y, nibble),
        _ => println!("UNKNOWN {:x}", opcode),
    }
}