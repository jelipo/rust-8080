use std::sync::{Arc, Mutex, RwLock};

use crate::memory::Memory;
use std::cell::RefCell;
use std::rc::Rc;

const RAM_SIZE: usize = 7168;

// Video RAM
pub struct Video {
    pub data: Rc<RefCell<[u8; RAM_SIZE]>>,
    /// 地址偏移量
    ofs: u16,
}

impl Memory for Video {
    fn get(&self, addr: u16) -> u8 {
        self.data.borrow()[(addr - self.ofs) as usize]
    }

    fn set(&mut self, addr: u16, val: u8) {
        //println!("设置Video {:X} {:X}", addr, val);
        if addr == 0x3DE1 {
            let a = 0;
        }
        self.data.borrow_mut()[(addr - self.ofs) as usize] = val;
    }
}

impl Video {
    pub fn init(ofs: u16, data: Rc<RefCell<[u8; 7168]>>) -> Video {
        Video {
            data,
            ofs,
        }
    }
}