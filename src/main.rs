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


fn main() {
    let launch = InvadersLaunch::new();
    launch.start();
}
