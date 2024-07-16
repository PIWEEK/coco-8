use coco_core::{Cpu, Machine};

#[derive(Debug)]
pub struct Vm {}

impl Machine for Vm {
    fn dei(&mut self, cpu: &mut Cpu, target: u8) {}
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> bool {
        false
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
