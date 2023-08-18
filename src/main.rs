use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.bf>", args[0]);
        std::process::exit(1);
    }
    let program = fs::read_to_string(&args[1])?;
    execute_brainfuck(&program)?;
    Ok(())
}

fn execute_brainfuck(program: &str) -> io::Result<()> {
    let mut memory = [0u8; 30000];
    let mut pointer = 0;
    let mut pc = 0;
    let mut loop_stack = Vec::new();

    while pc < program.len() {
        match program.chars().nth(pc).unwrap() {
            '+' => memory[pointer] = memory[pointer].wrapping_add(1),
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            '>' => pointer = pointer.wrapping_add(1),
            '<' => pointer = pointer.wrapping_sub(1),
            '[' => {
                loop_stack.push(pc);
            }
            ']' => {
                if memory[pointer] != 0 {
                    pc = *loop_stack.last().unwrap();
                } else {
                    loop_stack.pop();
                }
            }
            '.' => print!("{}", memory[pointer] as char),
            ',' => {
                io::stdout().flush()?;
                memory[pointer] = get_char()? as u8;
            }
            _ => {}
        }
        pc += 1;
    }
    Ok(())
}

fn get_char() -> io::Result<u8> {
    let mut buffer = [0u8; 1];
    io::stdin().read(&mut buffer)?;
    Ok(buffer[0])
}

