use super::Device;
use coco_core::Ports;

#[derive(Debug)]
pub struct SystemPorts {}

impl Ports for SystemPorts {
    const BASE: u8 = 0x00;
}

impl SystemPorts {
    const VECTOR: u8 = 0x00;
    const DEBUG: u8 = 0x02;
}

#[derive(Debug)]
pub struct SystemDevice {
    stdout: String,
}

impl SystemDevice {
    pub fn new() -> Self {
        Self {
            stdout: "".to_string(),
        }
    }

    pub fn debug(&mut self, cpu: &mut coco_core::Cpu) {
        let ports = cpu.device_page::<SystemPorts>();
        if !ports[SystemPorts::DEBUG as usize] == 0 {
            return;
        }

        // reset debug port to zero
        ports[SystemPorts::DEBUG as usize] = 0x00;

        // output debug info
        self.stdout += &format!("{}", cpu);
    }

    /// Returns the stdout buffer and flushes it
    pub fn stdout(&mut self) -> String {
        let res = self.stdout.to_owned();
        self.stdout = "".to_string();
        res
    }
}

impl Device for SystemDevice {
    fn deo(&mut self, cpu: &mut coco_core::Cpu, target: u8) {
        match target {
            SystemPorts::VECTOR => {}
            SystemPorts::DEBUG => self.debug(cpu),
            _ => {}
        }
    }

    fn dei(&mut self, _: &mut coco_core::Cpu, _: u8) {}
}
