use wasm_bindgen::prelude::*;

use coco_core::{Cpu, Machine};

pub type Result<T> = core::result::Result<T, String>;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug)]
pub struct Output {
    pub pc: u16,
}

#[wasm_bindgen(js_name=runRom)]
pub fn run_rom(rom: &[u8]) -> Result<Output> {
    let mut memory = [0; 0x10000];
    memory[0x100..(0x100 + rom.len())].copy_from_slice(rom);

    let mut cpu = Cpu::new(&mut memory);
    let mut vm = Vm::new();

    cpu.run(0x100, &mut vm);

    let output = Output { pc: cpu.pc() };
    println!("{:?}", output);

    Ok(output)
}

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
