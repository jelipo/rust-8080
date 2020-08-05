use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::Read;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use minifb::{Key, Window, WindowOptions};

use crate::cpu::Cpu;
use crate::memory::{AddressBus, SpaceInvadersAddressing, TestAddressing};
use crate::game::{InvadersLaunch, Launch};

mod util;

mod cpu;
mod memory;
mod game;

const WIDTH: usize = 256;
const HEIGHT: usize = 224;

fn main() {
    let launch = InvadersLaunch::new();
    launch.start();

    //
    // let video_arr = Rc::new(RefCell::new([0u8; 7168]));
    // let addressing = init_address(video_arr.clone()).unwrap();
    // let mut cpu = Cpu::new(Box::new(addressing), 0);
    //
    // //std::thread::sleep(Duration::from_secs(2));
    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    //
    // let mut window = Window::new(
    //     "Test - ESC to exit",
    //     WIDTH,
    //     HEIGHT,
    //     WindowOptions::default(),
    // ).unwrap_or_else(|e| {
    //     panic!("{}", e);
    // });
    //
    // // 限制最高60帧
    // window.limit_update_rate(Some(std::time::Duration::from_micros(8600)));
    //
    //
    // let mut int_num: bool = false;
    // let mut lasttime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - lasttime);
    //     lasttime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    //     for _i in 0..10000 {
    //         let op_code = cpu.next();
    //     }
    //
    //     let result = cpu.interrupt(if int_num { 0x10 } else { 0x08 });
    //     if result {
    //         int_num = !int_num;
    //     }
    //     for _i in 0..10000 {
    //         let op_code = cpu.next();
    //     }
    //     test(&mut buffer, video_arr.clone());
    //     window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    // }
}

fn get_color(bit: u8) -> u32 {
    return if bit == 0 { 0 } else { u32::max_value() };
}

fn cpu_debug(cpu: &mut Cpu) -> io::Result<()> {
    println!("   no        op       af      bc      de      hl      pc      sp  ");
    let mut op_code;
    let times = 0042050;
    for i in 0..times {
        op_code = cpu.next();
        if times - i < 100 {
            println!("{:07}:    {:#04X}     {:04X}    {:04X}    {:04X}    {:04X}    {:04X}    {:04X}",
                     i + 1, op_code, cpu.register.get_af(), cpu.register.get_bc(), cpu.register.get_de(),
                     cpu.register.get_hl(), cpu.register.pc, cpu.register.sp);
        }
    }

    Ok(())
}

// fn init_address(video_arr: Rc<RefCell<[u8; 7168]>>) -> io::Result<SpaceInvadersAddressing> {
//     let mut arr_h = [0u8; 2048];
//     let mut h = File::open("./res/invaders.h")?;
//     h.read(&mut arr_h)?;
//
//     let mut arr_g = [0u8; 2048];
//     let mut g = File::open("./res/invaders.g")?;
//     g.read(&mut arr_g)?;
//
//     let mut arr_f = [0u8; 2048];
//     let mut f = File::open("./res/invaders.f")?;
//     f.read(&mut arr_f)?;
//
//     let mut arr_e = [0u8; 2048];
//     let mut e = File::open("./res/invaders.e")?;
//     e.read(&mut arr_e)?;
//
//     let addressing = SpaceInvadersAddressing::new(
//         Box::new(arr_h), Box::new(arr_g), Box::new(arr_f), Box::new(arr_e), video_arr);
//     Ok(addressing)
// }

fn test(buffer: &mut Vec<u32>, video_arr: Rc<RefCell<[u8; 7168]>>) {
    for i in 0..buffer.len() {
        let line = i / 8;
        if (i % 8) != 0 {
            continue;
        }
        let byte = video_arr.borrow_mut()[line as usize];
        buffer[i] = get_color(byte & 0b0000_0001);
        buffer[i + 1] = get_color(byte & 0b0000_0010);
        buffer[i + 2] = get_color(byte & 0b0000_0100);
        buffer[i + 3] = get_color(byte & 0b0000_1000);
        buffer[i + 4] = get_color(byte & 0b0001_0000);
        buffer[i + 5] = get_color(byte & 0b0010_0000);
        buffer[i + 6] = get_color(byte & 0b0100_0000);
        buffer[i + 7] = get_color(byte & 0b1000_0000);
    }
}

// fn cpu_test() {
//     let mut f = File::open("C:/Users/cao/Desktop/cpudiag.bin").expect("Failed to open cpu.bin");
//     let mut buffer = Vec::new();
//     f.read_to_end(&mut buffer)
//         .expect("Failed to read cpu.bin into buffer");
//     let addressing = TestAddressing::new(&mut buffer);
//     let mut state = Cpu::new(Box::new(addressing), 0x100);
//     let mut a: u64 = 0;
//     loop {
//         let i = state.next();
//         if i != 0 {
//             println!("{:X} times:{} ", i, a);
//         }
//         a += 1;
//     }
// }
