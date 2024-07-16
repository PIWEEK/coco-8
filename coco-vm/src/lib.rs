use coco_core::{Cpu, Machine};

pub const SCREEN_WIDTH: usize = 192;
pub const SCREEN_HEIGHT: usize = 144;
const VIDEO_BUFFER_LEN: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub type Pixel = u8;

#[derive(Debug, Clone, Copy)]
pub struct Output {
    pub pc: u16,
}

#[derive(Debug)]
pub struct Vm {
    video: VideoDevice,
}

impl Machine for Vm {
    fn dei(&mut self, cpu: &mut Cpu, target: u8) {}
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> bool {
        false
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {
            video: VideoDevice::new(),
        }
    }

    pub fn on_reset(&mut self, cpu: &mut Cpu) -> Output {
        cpu.run(0x100, self);
        Output { pc: cpu.pc() }
    }

    pub fn on_video(&mut self, cpu: &mut Cpu) -> Output {
        // TODO: call video vector
        cpu.run(0x100, self);

        Output { pc: cpu.pc() }
    }

    pub fn pixels(&self) -> (&[Pixel; VIDEO_BUFFER_LEN], &[Pixel; VIDEO_BUFFER_LEN]) {
        (&self.video.background, &self.video.foreground)
    }
}
#[derive(Debug)]
struct VideoDevice {
    background: [Pixel; VIDEO_BUFFER_LEN],
    foreground: [Pixel; VIDEO_BUFFER_LEN],
}

impl VideoDevice {
    pub fn new() -> Self {
        let mut buffer = [0x00 as Pixel; VIDEO_BUFFER_LEN];
        for i in 0..VIDEO_BUFFER_LEN {
            buffer[i] = (i % 0x10) as Pixel;
        }
        Self {
            background: [0x00; VIDEO_BUFFER_LEN],
            foreground: buffer,
        }
    }
}
