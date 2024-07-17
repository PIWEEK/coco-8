mod system;
mod video;

use coco_core::{Cpu, Machine, Ports};
use system::{SystemDevice, SystemPorts};
use video::{VideoBuffer, VideoDevice, VideoPorts};

pub use video::{SCREEN_HEIGHT, SCREEN_WIDTH};

trait Device {
    #[allow(dead_code)]
    fn dei(&mut self, cpu: &mut Cpu, target: u8);
    fn deo(&mut self, cpu: &mut Cpu, target: u8);
}

#[derive(Debug, Clone)]
pub struct DeviceOutput {
    pub shall_halt: bool,
    pub sys_stdout: String,
}

impl core::default::Default for DeviceOutput {
    fn default() -> Self {
        DeviceOutput {
            shall_halt: false,
            sys_stdout: String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct Vm {
    video: VideoDevice,
    system: SystemDevice,
}

impl Machine for Vm {
    fn dei(&mut self, _: &mut Cpu, _: u8) {}
    fn deo(&mut self, cpu: &mut Cpu, target: u8) {
        let offset = target & 0x0f;
        match target & 0xf0 {
            SystemPorts::BASE => self.system.deo(cpu, offset),
            VideoPorts::BASE => self.video.deo(cpu, offset),
            _ => unimplemented!(),
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
        self.output()
    }

    pub fn on_video(&mut self, cpu: &mut Cpu) -> DeviceOutput {
        // TODO: call video vector
        cpu.run(0x200, self);
        self.output()
    }

    pub fn pixels(&self) -> (&VideoBuffer, &VideoBuffer) {
        (&self.video.background, &self.video.foreground)
    }

    pub fn output(&mut self) -> DeviceOutput {
        DeviceOutput {
            shall_halt: false,
            sys_stdout: self.system.stdout(),
        }
    }
}
