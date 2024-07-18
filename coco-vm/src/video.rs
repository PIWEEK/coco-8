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

pub const SCREEN_WIDTH: u8 = 192;
pub const SCREEN_HEIGHT: u8 = 144;
pub const VIDEO_BUFFER_LEN: usize = SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize;

pub type Pixel = u8;
pub type VideoBuffer = [Pixel; VIDEO_BUFFER_LEN];

#[derive(Debug)]
pub struct VideoDevice {
    pub layers: [VideoBuffer; 2],
    is_dirty: bool,
    buffer: VideoBuffer,
}

impl VideoDevice {
    pub fn new() -> Self {
        Self {
            layers: [[0x00; VIDEO_BUFFER_LEN]; 2],
            is_dirty: true,
            buffer: [0x00; VIDEO_BUFFER_LEN],
        }
    }

    pub fn pixels(&mut self) -> &VideoBuffer {
        if std::mem::take(&mut self.is_dirty) {
            self.refresh_buffer();
        }

        return &self.buffer;
    }

    fn refresh_buffer(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = if self.layers[0x01][i] != 0x00 {
                self.layers[0x01][i]
            } else {
                self.layers[0x00][i]
            }
        }
    }

    #[inline]
    fn xy(&self, ports: &mut [u8]) -> (u8, u8) {
        let x = cmp::min(ports[VideoPorts::X as usize], (SCREEN_WIDTH - 1) as u8);
        let y = cmp::min(ports[VideoPorts::Y as usize], (SCREEN_HEIGHT - 1) as u8);
        (x, y)
    }

    fn deo_pixel(&mut self, cpu: &mut Cpu) {
        self.is_dirty = true;

        let ports = cpu.device_page::<VideoPorts>();
        let pixel = ports[VideoPorts::PIXEL as usize];

        let (x, y) = self.xy(ports);
        let color = pixel & 0x0f;
        let layer = (pixel & 0b0001_0000) >> 4;
        let is_fill = ((pixel & 0b0010_0000) >> 5) == 0x01;

        if is_fill {
            let is_flip_x = ((pixel & 0b1000_0000) >> 7) == 0x01;
            let is_flip_y = ((pixel & 0b0100_0000) >> 6) == 0x01;
            self.fill(x, y, color, layer, is_flip_x, is_flip_y);
        } else {
            self.put_pixel(x, y, color, layer);
        }
    }

    fn fill(&mut self, x: u8, y: u8, color: Pixel, layer: u8, is_flip_x: bool, is_flip_y: bool) {
        let start_x = if is_flip_x { 0 } else { x };
        let end_x = if is_flip_x { x } else { SCREEN_WIDTH - 1 };
        let start_y = if is_flip_y { 0 } else { y };
        let end_y = if is_flip_y { y } else { SCREEN_HEIGHT - 1 };

        for col in start_x..=end_x {
            for row in start_y..=end_y {
                self.put_pixel(col, row, color, layer);
            }
        }
    }

    #[inline]
    fn put_pixel(&mut self, x: u8, y: u8, color: u8, layer: u8) {
        let i = y as usize * SCREEN_WIDTH as usize + x as usize;
        self.layer(layer)[i] = color;
    }

    #[inline]
    fn layer(&mut self, i: u8) -> &mut VideoBuffer {
        &mut self.layers[i as usize]
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
