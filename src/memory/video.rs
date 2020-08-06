


use std::sync::{Arc, RwLock};

use crate::memory::Memory;

const RAM_SIZE: usize = 7168;

// Video RAM
pub struct Video {
    pub data: Arc<RwLock<Vec<u8>>>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        self.data.read().unwrap()[(addr - self.ofs) as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        self.data.write().unwrap()[(addr - self.ofs) as usize] = val;
    }
}

impl Video {
    pub fn init(ofs: u16, data: Arc<RwLock<Vec<u8>>>) -> Video {
        Video {
            data,
            ofs,
        }
    }
}