use wasm_bindgen::prelude::*;

use coco_core::Cpu;
use coco_vm::Vm;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Output {
    pub pc: u16,
}

pub type Result<T> = core::result::Result<T, JsValue>;

#[wasm_bindgen(js_name=runRom)]
pub fn run_rom(rom: &[u8]) -> Result<Output> {
    let mut memory = [0; 0x10000];
    memory[0x100..(0x100 + rom.len())].copy_from_slice(rom);

    let mut cpu = Cpu::new(&mut memory);
    let mut vm = Vm::new();

    // run reset vector
    cpu.run(0x100, &mut vm);

    Ok(Output { pc: cpu.pc() })
}
