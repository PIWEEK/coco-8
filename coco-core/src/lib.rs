#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

use core::fmt;

mod stack;
use stack::Stack;

mod opcodes;

/// The trait to implement for COCO virtual machines.
pub trait Machine {
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> bool;
    fn dei(&mut self, cpu: &mut Cpu, target: u8);
}

/// The trait to implement a COCO device's ports
pub trait Ports {
    const BASE: u8;
}

/// COCO-8 CPU.
#[derive(Debug)]
pub struct Cpu {
    /// Main memory (64 Kb)
    ram: [u8; 0x10000],
    /// Device page (256 bytes)
    devices: [u8; 0x100],
    /// Main, working stack (256 bytes)
    stack: Stack,
    /// Return stack (256 bytes)
    ret_stack: Stack,
    /// Program counter
    pc: u16,
}

impl Cpu {
    /// Returns a new CPU with their memory, stacks and PC reset to zero.
    pub fn new(rom: &[u8]) -> Self {
        // load rom at address 0x100
        let mut ram = [0; 0x10000];
        ram[0x100..].copy_from_slice(rom);

        Self {
            ram,
            devices: [0; 0x100],
            stack: Stack::new(),
            ret_stack: Stack::new(),
            pc: 0,
        }
    }

    /// Runs the code starting the PC in the given address until
    /// it finds a BRK opcode
    pub fn run<M: Machine>(&mut self, addr: u16, _: &mut M) -> u16 {
        self.pc = addr;
        loop {
            let op = self.read_byte();
            match op {
                opcodes::BRK => break,
                opcodes::PUSH => self.op_push(),
                _ => {}
            }
        }

        self.pc
    }

    /// Returns the requested device page
    #[inline]
    pub fn device_page<D: Ports>(&mut self) -> &mut [u8] {
        &mut self.devices[(D::BASE as usize)..(D::BASE as usize + 0x10)]
    }

    /// Returns the current value for the program counter (PC)
    pub fn pc(&self) -> u16 {
        self.pc
    }

    #[inline]
    fn read_byte(&mut self) -> u8 {
        let res = self.ram[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        res
    }

    #[inline]
    fn op_push(&mut self) {
        let value = self.read_byte();
        self.stack.push_byte(value);
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "WRK: [{}]", self.stack)?;
        write!(f, "RET: [{}]", self.ret_stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct AnyMachine {}
    impl Machine for AnyMachine {
        fn deo(&mut self, _: &mut Cpu, _: u8) -> bool {
            false
        }
        fn dei(&mut self, _: &mut Cpu, _: u8) {}
    }

    fn zeroed_memory() -> [u8; 0x10000 - 0x100] {
        [0_u8; 0x10000 - 0x100]
    }

    fn rom_from(rom: &[u8]) -> [u8; 0x10000 - 0x100] {
        let mut res = [0_u8; 0x10000 - 0x100];
        res[0..rom.len()].copy_from_slice(rom);
        res
    }

    #[test]
    fn creates_cpu() {
        let memory = zeroed_memory();
        let cpu = Cpu::new(&memory);

        assert_eq!(cpu.pc, 0);
    }

    #[test]
    pub fn runs_until_break() {
        let rom = rom_from(&[0x01, 0x01, 0x00]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x103);
        assert_eq!(pc, cpu.pc);
    }

    #[test]
    pub fn run_wraps_pc_at_the_end_of_ram() {
        let mut rom = zeroed_memory();
        rom[rom.len() - 1] = 0x01;
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0xffff, &mut AnyMachine {});

        assert_eq!(pc, 0x01);
        assert_eq!(pc, cpu.pc);
    }

    #[test]
    pub fn push_opcode() {
        let rom = rom_from(&[0x80, 0xab, 0x00]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});
        assert_eq!(pc, 0x103);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xab);
    }
}
