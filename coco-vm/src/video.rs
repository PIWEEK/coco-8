use core::cmp;

use super::Device;
use coco_core::{Cpu, Ports};

#[derive(Debug)]
pub struct VideoPorts {}

impl Ports for VideoPorts {
    const BASE: u8 = 0x10;
}

impl VideoPorts {
    #[allow(dead_code)]
    const VECTOR: u8 = 0x00;
    const X: u8 = 0x02;
    const Y: u8 = 0x03;
    const PIXEL: u8 = 0x04;
}

pub const SCREEN_WIDTH: usize = 192;
pub const SCREEN_HEIGHT: usize = 144;
const VIDEO_BUFFER_LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub type Pixel = u8;
pub type VideoBuffer = [Pixel; VIDEO_BUFFER_LEN];

#[derive(Debug)]
pub struct VideoDevice {
    pub background: [Pixel; VIDEO_BUFFER_LEN],
    pub foreground: [Pixel; VIDEO_BUFFER_LEN],
}

impl VideoDevice {
    pub fn new() -> Self {
        Self {
            background: [0x00; VIDEO_BUFFER_LEN],
            foreground: [0x00; VIDEO_BUFFER_LEN],
        }
    }

    #[inline]
    fn xy(&self, ports: &mut [u8]) -> (u8, u8) {
        let x = cmp::min(ports[VideoPorts::X as usize], (SCREEN_WIDTH - 1) as u8);
        let y = cmp::min(ports[VideoPorts::Y as usize], (SCREEN_HEIGHT - 1) as u8);
        (x, y)
    }

    fn deo_pixel(&mut self, cpu: &mut Cpu) {
        let ports = cpu.device_page::<VideoPorts>();
        let pixel = ports[VideoPorts::PIXEL as usize];

        let (x, y) = self.xy(ports);
        let color = pixel & 0x0f;
        let is_foreground = ((pixel & 0b0001_0000) >> 4) == 0x01;
        let is_fill = ((pixel & 0b0010_0000) >> 5) == 0x01;

        if is_fill {
            self.fill(x, y, color, is_foreground);
        } else {
            self.put_pixel(x, y, color, is_foreground);
        }
    }

    fn fill(&mut self, x: u8, y: u8, color: Pixel, is_foreground: bool) {
        let chunk = vec![color; SCREEN_WIDTH - x as usize];
        let layer = self.layer(is_foreground);

        for row in (y as usize)..SCREEN_HEIGHT {
            let i = x as usize + row as usize * SCREEN_WIDTH;
            layer[i..(i + chunk.len())].copy_from_slice(&chunk);
        }
    }

    #[inline]
    fn layer(&mut self, is_foreground: bool) -> &mut [Pixel; VIDEO_BUFFER_LEN] {
        if is_foreground {
            &mut self.foreground
        } else {
            &mut self.background
        }
    }

    #[inline]
    fn put_pixel(&mut self, x: u8, y: u8, color: u8, is_foreground: bool) {
        let i = y as usize * SCREEN_WIDTH + x as usize;

        if is_foreground {
            self.foreground[i] = color;
        } else {
            self.background[i] = color;
        }
    }
}

impl Device for VideoDevice {
    fn deo(&mut self, cpu: &mut Cpu, target: u8) {
        match target {
            VideoPorts::X => {}
            VideoPorts::Y => {}
            VideoPorts::PIXEL => self.deo_pixel(cpu),
            _ => {}
        }
    }

    fn dei(&mut self, _: &mut Cpu, _: u8) {}
}
