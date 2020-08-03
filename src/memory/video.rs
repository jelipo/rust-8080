use std::sync::{Arc, Mutex, RwLock};

use crate::memory::Memory;

const RAM_SIZE: usize = 7168;

// Video RAM
pub struct Video {
    pub data: Arc<RwLock<[u8; RAM_SIZE]>>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        self.data.read().unwrap()[(addr - self.ofs) as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        println!("设置Video {:X} {:X}", addr, val);
        if addr == 0x3DE1 {
            let a = 0;
        }
        let mut write = self.data.write().unwrap();
        write[(addr - self.ofs) as usize] = val
    }
}

impl Video {
    pub fn init(ofs: u16, data: Arc<RwLock<[u8; 7168]>>) -> Video {
        Video {
            data,
            ofs,
        }
    }
}