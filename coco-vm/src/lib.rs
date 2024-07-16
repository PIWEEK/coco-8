mod video;

use coco_core::{Cpu, Machine};
use video::{VideoBuffer, VideoDevice};

pub use video::{SCREEN_HEIGHT, SCREEN_WIDTH};

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

    pub fn pixels(&self) -> (&VideoBuffer, &VideoBuffer) {
        (&self.video.background, &self.video.foreground)
    }
}
