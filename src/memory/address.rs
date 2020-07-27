use std::cell::{RefCell, RefMut};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::rc::Rc;

use crate::memory::{Memory, ReadOnly, Video, Work};

pub trait Addressing {
    fn get_mem(&self, addr: u16) -> u8;

    fn set_mem(&mut self, addr: u16, val: u8);
}