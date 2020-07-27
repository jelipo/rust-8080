use std::fs::File;
use std::io;
use std::io::Read;

mod util;

mod cpu;
mod memory;

fn main() -> io::Result<()> {
    let mut a: u8 = 0xFA;
    for i in 0..10 {
        a = a.wrapping_add(1);
        let b = 0;
    }
    Ok(())
}
