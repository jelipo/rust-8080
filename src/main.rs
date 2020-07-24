use std::fs::File;
use std::io;
use std::io::Read;

mod motherboard;
mod cpu;
mod memory;

fn main() -> io::Result<()> {
    let mut file = File::open("C:/Users/cao/Desktop/pokemon-yellow.gbc")?;
    let mut x = [0u8; 2048];
    let x1 = file.read_exact(&mut x)?;
    let x2 = file.read_exact(&mut x)?;
    let a = 0;
    Ok(())
}
