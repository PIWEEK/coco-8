use core::cmp;

use super::Device;
use coco_core::{Cpu, Ports};

#[derive(Debug)]
pub struct VideoPorts {}

impl Ports for VideoPorts {
    const BASE: u8 = 0x10;
}

impl VideoPorts {
    const VECTOR: u8 = 0x00;
    const X: u8 = 0x02;
    const Y: u8 = 0x03;
    const PIXEL: u8 = 0x04;
    const ADDRESS: u8 = 0x08;
    const SPRITE: u8 = 0x0a;
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
    vector: u16,
}

impl VideoDevice {
    pub fn new() -> Self {
        Self {
            layers: [[0x00; VIDEO_BUFFER_LEN]; 2],
            is_dirty: true,
            buffer: [0x00; VIDEO_BUFFER_LEN],
            vector: 0,
        }
    }

    pub fn vector(&self) -> u16 {
        self.vector
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
    fn xy(&self, ports: &[u8]) -> (u8, u8) {
        let x = cmp::min(ports[VideoPorts::X as usize], (SCREEN_WIDTH - 1) as u8);
        let y = cmp::min(ports[VideoPorts::Y as usize], (SCREEN_HEIGHT - 1) as u8);
        (x, y)
    }

    #[inline]
    fn address(&self, ports: &[u8]) -> u16 {
        let hi = ports[VideoPorts::ADDRESS as usize];
        let lo = ports[VideoPorts::ADDRESS.wrapping_add(1) as usize];

        u16::from_be_bytes([hi, lo])
    }

    #[inline]
    fn deo_vector(&mut self, cpu: &mut Cpu) {
        let ports = cpu.device_page::<VideoPorts>();
        let hi = ports[VideoPorts::VECTOR as usize];
        let lo = ports[VideoPorts::VECTOR as usize + 1];

        self.vector = u16::from_be_bytes([hi, lo]);
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

    fn deo_sprite(&mut self, cpu: &mut Cpu) {
        self.is_dirty = true;
        let ports = cpu.device_page::<VideoPorts>();
        let sprite_port = ports[VideoPorts::SPRITE as usize];

        let (x, y) = self.xy(ports);
        let addr = self.address(ports);
        let sprite_data = self.sprite_data(addr, cpu);
        let layer = (sprite_port & 0b0001_0000) >> 4;

        for spr_y in 0..8 {
            for spr_x in 0..8 {
                let spr_pixel = sprite_data[spr_y as usize * 8 + spr_x as usize];
                let _x = x + spr_x;
                let _y = y + spr_y;

                if _x >= SCREEN_WIDTH || _y >= SCREEN_HEIGHT {
                    continue;
                }
                self.put_pixel(_x, _y, spr_pixel, layer);
            }
        }
    }

    fn sprite_data(&self, base_addr: u16, cpu: &Cpu) -> [Pixel; 64] {
        let mut addr = base_addr;
        let mut res = [0x00; 64];
        for row in 0..8 as usize {
            for chunk in 0..4 as usize {
                let pixel_data = cpu.ram_peek_byte(addr.wrapping_add(chunk as u16));
                res[row * 8 + chunk * 2 + 0] = (0b1111_0000 & pixel_data) >> 4;
                res[row * 8 + chunk * 2 + 1] = 0b0000_1111 & pixel_data;
            }
            addr = addr.wrapping_add(4);
        }
        res
    }

    #[inline]
    fn layer(&mut self, i: u8) -> &mut VideoBuffer {
        &mut self.layers[i as usize]
    }
}

impl Device for VideoDevice {
    fn deo(&mut self, cpu: &mut Cpu, target: u8) {
        match target {
            VideoPorts::VECTOR => self.deo_vector(cpu),
            VideoPorts::X => {}
            VideoPorts::Y => {}
            VideoPorts::PIXEL => self.deo_pixel(cpu),
            VideoPorts::ADDRESS => {}
            VideoPorts::SPRITE => {
                self.deo_sprite(cpu);
            }
            _ => {}
        }
    }

    fn dei(&mut self, _: &mut Cpu, _: u8) {}
}
