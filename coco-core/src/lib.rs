#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

/// The trait to implement for COCO virtual machines.
pub trait Machine {
    fn deo(&mut self, cpu: &mut Cpu, target: u8) -> bool;
    fn dei(&mut self, cpu: &mut Cpu, target: u8);
}

#[derive(Debug)]
struct Stack {
    data: [u8; 0x100],
    index: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0_u8; 0x100],
            index: 0,
        }
    }
}
mod opcodes {
    pub const BRK: u8 = 0x00;
}

/// COCO-8 CPU.
pub struct Cpu<'a> {
    /// Main memory (64 Kb)
    ram: &'a mut [u8; 0x10000],
    /// Main, working stack (256 bytes)
    stack: Stack,
    /// Return stack (256 bytes)
    ret_stack: Stack,
    /// Program counter
    pc: u16,
}

impl<'a> Cpu<'a> {
    /// Returns a new CPU with their memory, stacks and PC reset to zero.
    pub fn new(ram: &'a mut [u8; 0x10000]) -> Self {
        Self {
            ram,
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
                opcodes::BRK => {
                    break;
                }
                _ => {}
            }
        }

        self.pc
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

    fn zeroed_memory() -> [u8; 0x10000] {
        [0_u8; 0x10000]
    }

    fn rom_from(rom: &[u8]) -> [u8; 0x10000] {
        let mut res = [0_u8; 0x10000];
        res[0..rom.len()].copy_from_slice(rom);
        res
    }

    #[test]
    fn creates_cpu() {
        let mut memory = zeroed_memory();
        let cpu = Cpu::new(&mut memory);

        assert_eq!(cpu.pc, 0);
    }

    #[test]
    pub fn runs_until_break() {
        let mut rom = rom_from(&[0x01, 0x01, 0x00]);
        let mut cpu = Cpu::new(&mut rom);

        let pc = cpu.run(0x00, &mut AnyMachine {});

        assert_eq!(pc, 0x03);
        assert_eq!(pc, cpu.pc);
    }

    #[test]
    pub fn run_wraps_pc_at_the_end_of_ram() {
        let mut rom = zeroed_memory();
        rom[0xffff] = 0x01;
        let mut cpu = Cpu::new(&mut rom);

        let pc = cpu.run(0xffff, &mut AnyMachine {});

        assert_eq!(pc, 0x01);
        assert_eq!(pc, cpu.pc);
    }
}
