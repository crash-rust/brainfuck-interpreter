use brainfuck_interpreter::Interpreter;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    let mut f = File::open(&args[1])?;
    let mut c: Vec<u8> = Vec::new();
    f.read_to_end(&mut c)?;
    let mut i = Interpreter::default();

    i.run(c)
}
