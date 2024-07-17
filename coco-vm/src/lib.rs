mod system;
mod video;

use coco_core::{Cpu, Machine};
use system::SystemDevice;
use video::{VideoBuffer, VideoDevice};

pub use video::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug)]
pub struct Vm {
    video: VideoDevice,
    system: SystemDevice,
}

impl Machine for Vm {
    fn dei(&mut self, cpu: &mut Cpu, target: u8) {}
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> bool {
        false
    }
}

trait Device {
    fn dei(&mut self, cpu: &mut Cpu, target: u8);
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> DeviceOutput;
}

pub struct DeviceOutput {
    pub shall_halt: bool,
    pub message: Option<String>,
}

impl core::default::Default for DeviceOutput {
    fn default() -> Self {
        DeviceOutput {
            shall_halt: false,
            message: None,
        }
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {
            video: VideoDevice::new(),
            system: SystemDevice::new(),
        }
    }

    pub fn on_reset(&mut self, cpu: &mut Cpu) -> DeviceOutput {
        cpu.run(0x100, self);

        DeviceOutput {
            shall_halt: false,
            message: Some(format!("{}", cpu)),
        }
    }

    pub fn on_video(&mut self, cpu: &mut Cpu) -> DeviceOutput {
        // TODO: call video vector
        cpu.run(0x100, self);

        DeviceOutput {
            shall_halt: false,
            message: Some(format!("{}", cpu)),
        }
    }

    pub fn pixels(&self) -> (&VideoBuffer, &VideoBuffer) {
        (&self.video.background, &self.video.foreground)
    }
}
