use crate::memory::{Addressing, Memory, ReadOnly, Video, Work};

pub struct SpaceInvadersAddressing {
    read_only_h: ReadOnly,
    read_only_g: ReadOnly,
    read_only_f: ReadOnly,
    read_only_e: ReadOnly,
    work_ram: Work,
    video_ram: Video,
}


impl Addressing for SpaceInvadersAddressing {
    fn get_mem(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x07ff => self.read_only_h.get(addr),
            0x0800..=0x0fff => self.read_only_g.get(addr),
            0x1000..=0x17ff => self.read_only_f.get(addr),
            0x1800..=0x1fff => self.read_only_e.get(addr),
            0x2000..=0x23ff => self.work_ram.get(addr),
            0x2400..=0x3fff => self.video_ram.get(addr),
            _ => panic!("address not support")
        }
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        //println!("{:X} 设置 {:X}", addr, val);

        match addr {
            0x0000..=0x07ff => self.read_only_h.set(addr, val),
            0x0800..=0x0fff => self.read_only_g.set(addr, val),
            0x1000..=0x17ff => self.read_only_f.set(addr, val),
            0x1800..=0x1fff => self.read_only_e.set(addr, val),
            0x2000..=0x23ff => self.work_ram.set(addr, val),
            0x2400..=0x3fff => self.video_ram.set(addr, val),
            _ => panic!("address not support")
        }
    }
}

impl SpaceInvadersAddressing {
    pub fn new(h_arr: Box<[u8; 2048]>,
               g_arr: Box<[u8; 2048]>,
               f_arr: Box<[u8; 2048]>,
               e_arr: Box<[u8; 2048]>,
    ) -> Self {
        Self {
            read_only_h: ReadOnly::init(0, h_arr),
            read_only_g: ReadOnly::init(0x0800, g_arr),
            read_only_f: ReadOnly::init(0x1000, f_arr),
            read_only_e: ReadOnly::init(0x1800, e_arr),
            work_ram: Work::init(0x2000, Box::new([0u8; 1024])),
            video_ram: Video::init(0x2400, Box::new([0u8; 7168])),
        }
    }
}