use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

pub struct Video {
    window: Window,
    buffer: Vec<u32>,
    video_arr: Arc<RwLock<Vec<u8>>>,
}

const GAME_NAME: &str = "invaders";
const WIDTH: usize = 256;
const HEIGHT: usize = 224;

impl Video {
    pub fn new(video_arr: Arc<RwLock<Vec<u8>>>) -> Self {
        let mut window = Window::new(
            format!("{} - Powered by Jelipo", GAME_NAME).as_str(),
            WIDTH, HEIGHT,
            WindowOptions {
                borderless: true,
                transparency: false,
                title: true,
                resize: false,
                scale: Scale::X2,
                scale_mode: ScaleMode::Stretch,
                topmost: false,
            },
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });
        Self {
            window,
            buffer: vec![0; WIDTH * HEIGHT],
            video_arr,
        }
    }

    /// This is block method
    pub fn start(&mut self) {
        // 限制最高60帧
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(8600)));

        let mut lasttime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            //println!("{:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - lasttime);
            lasttime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();


            self.display();
        }
    }

    fn display(&mut self) {
        for i in 0..self.buffer.len() {
            let line = i / 8;
            if (i % 8) != 0 { continue; }
            let byte = self.video_arr.read().unwrap()[line as usize];
            self.buffer[i] = get_color(byte & 0b0000_0001);
            self.buffer[i + 1] = get_color(byte & 0b0000_0010);
            self.buffer[i + 2] = get_color(byte & 0b0000_0100);
            self.buffer[i + 3] = get_color(byte & 0b0000_1000);
            self.buffer[i + 4] = get_color(byte & 0b0001_0000);
            self.buffer[i + 5] = get_color(byte & 0b0010_0000);
            self.buffer[i + 6] = get_color(byte & 0b0100_0000);
            self.buffer[i + 7] = get_color(byte & 0b1000_0000);
        }
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn get_color(bit: u8) -> u32 {
    return if bit == 0 { 0 } else { u32::max_value() };
}


