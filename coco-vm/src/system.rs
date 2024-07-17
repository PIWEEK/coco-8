use crate::DeviceOutput;

use super::Device;
use coco_core::Ports;

#[derive(Debug)]
pub struct SystemPorts {}

impl Ports for SystemPorts {
    const BASE: u8 = 0x00;
}

impl SystemPorts {
    const DEBUG: u8 = 0x00;
}

#[derive(Debug)]
pub struct SystemDevice {}

impl SystemDevice {
    pub fn new() -> Self {
        Self {}
    }

    pub fn debug(&self, cpu: &mut coco_core::Cpu) -> DeviceOutput {
        let ports = cpu.device_page::<SystemPorts>();

        if !ports[SystemPorts::DEBUG as usize] > 0 {
            return DeviceOutput::default();
        }

        // reset debug port to zero
        ports[SystemPorts::DEBUG as usize] = 0x00;

        // output debug info
        let mut res = DeviceOutput::default();
        res.message = Some(format!("{}", cpu));
        res
    }
}

impl Device for SystemDevice {
    fn deo(&mut self, cpu: &mut coco_core::Cpu, target: u8) -> DeviceOutput {
        match target {
            SystemPorts::DEBUG => self.debug(cpu),
            _ => panic!("Unimplemented device port {}", target),
        }
    }

    fn dei(&mut self, cpu: &mut coco_core::Cpu, target: u8) {}
}
