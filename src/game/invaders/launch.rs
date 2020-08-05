use std::{io, thread};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::cpu::Cpu;
use crate::game::invaders::video::Video;
use crate::game::Launch;
use crate::memory::SpaceInvadersAddressing;

pub struct InvadersLaunch {}

impl Launch for InvadersLaunch {
    fn start(&self) {
        let gpu_ram = vec![0u8; 7168];
        //let gpu_ram_mut = Rc::new(RefCell::new(gpu_ram));
        // let arc = Arc::new(RwLock::new(gpu_ram));
        // //let gpu_ram_rc = Rc::new(gpu_ram);
        // let arc_cloned = arc.clone();


        let video_arr = Arc::new(RwLock::new(gpu_ram));
        let video_arr_cloned = video_arr.clone();
        thread::spawn(move || {
            let addressing = init_address(video_arr_cloned.clone()).unwrap();
            let mut cpu = Cpu::new(Box::new(addressing), 0);
            let mut int_num: bool = false;
            let cycle_frame: u32 = 333333;
            let mut cycle_frame_temp: u32 = 0;
            let mut last_sleep = get_mill_time();
            loop {
                let first = get_mill_time();
                loop {
                    let cycle = cpu.next();
                    cycle_frame_temp += (cycle as u32);
                    if cycle_frame_temp > cycle_frame {
                        cycle_frame_temp = 0;
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(16));
                let result = cpu.interrupt(if int_num { 0x10 } else { 0x08 });
                if result {
                    int_num = !int_num;
                }
                // if cycle_frame_temp > cycle_frame {
                //     cycle_frame_temp = 0;
                //     // let last_sleep_times = get_mill_time() - last_sleep;
                //     // let sleep_time = (16 as u128).wrapping_sub(last_sleep_times);
                //     // if sleep_time <= 16 {
                //     //     thread::sleep(Duration::from_millis(16));
                //     // }
                //     // last_sleep = get_mill_time();
                //     thread::sleep(Duration::from_millis(16));
                // }
            }
        });
        let mut video = Video::new(video_arr.clone());
        video.start();
    }
}

fn get_mill_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

impl InvadersLaunch {
    pub fn new() -> Self {
        Self {}
    }
}

fn init_address(video_arr: Arc<RwLock<Vec<u8>>>) -> io::Result<SpaceInvadersAddressing> {
    let mut arr_h = [0u8; 2048];
    let mut h = File::open("./res/invaders.h")?;
    h.read(&mut arr_h)?;

    let mut arr_g = [0u8; 2048];
    let mut g = File::open("./res/invaders.g")?;
    g.read(&mut arr_g)?;

    let mut arr_f = [0u8; 2048];
    let mut f = File::open("./res/invaders.f")?;
    f.read(&mut arr_f)?;

    let mut arr_e = [0u8; 2048];
    let mut e = File::open("./res/invaders.e")?;
    e.read(&mut arr_e)?;

    let addressing = SpaceInvadersAddressing::new(
        Box::new(arr_h), Box::new(arr_g), Box::new(arr_f), Box::new(arr_e), video_arr);
    Ok(addressing)
}