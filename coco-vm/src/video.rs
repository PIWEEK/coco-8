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
        (ports[VideoPorts::X as usize], ports[VideoPorts::Y as usize])
    }

    fn deo_pixel(&mut self, cpu: &mut Cpu) {
        let ports = cpu.device_page::<VideoPorts>();
        let pixel = ports[VideoPorts::PIXEL as usize];

        let (x, y) = self.xy(ports);
        let color = pixel & 0x0f;
        let layer = (pixel & 0b0001_0000) >> 4;

        self.put_pixel(x, y, color, layer == 0x01);
    }

    fn put_pixel(&mut self, x: u8, y: u8, color: u8, is_foreground: bool) {
        let x = cmp::min(x as usize, SCREEN_WIDTH - 1);
        let y = cmp::min(y as usize, SCREEN_HEIGHT - 1);
        let i = y * SCREEN_WIDTH + x;

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
