#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]

use core::cmp;
use core::fmt;

mod stack;
use opcodes::short_mode;
use stack::Stack;

pub mod opcodes;
use opcodes::FLAG_SHORT;

/// The trait to implement for COCO virtual machines.
pub trait Machine {
    fn deo(&mut self, cpu: &mut Cpu, target: u8);
    fn dei(&mut self, cpu: &mut Cpu, target: u8);
}

/// The trait to implement a COCO device's ports
pub trait Ports {
    const BASE: u8;
}

macro_rules! binary_op {
    ($self:ident, $flags:ident, $f:expr) => {
        if short_mode($flags) {
            let b = $self.stack.pop_short();
            let a = $self.stack.pop_short();
            let f: fn(u16, u16) -> u16 = $f;
            $self.stack.push_short(f(a, b))
        } else {
            let b = $self.stack.pop_byte();
            let a = $self.stack.pop_byte();
            let f: fn(u8, u8) -> u8 = $f;
            $self.stack.push_byte(f(a, b))
        }
    };
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
        ram[0x100..cmp::min(0x100 + rom.len(), 0x10000)].copy_from_slice(rom);

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
    pub fn run(&mut self, addr: u16, machine: &mut impl Machine) -> u16 {
        self.pc = addr;
        loop {
            let op = self.read_byte();
            match op {
                opcodes::BRK => break,
                opcodes::INC => self.op_inc(),
                opcodes::DUP => self.op_dup(),
                opcodes::DUP2 => self.op_dup2(),
                opcodes::EQU => self.op_equ::<0x00>(),
                opcodes::EQU2 => self.op_equ::<FLAG_SHORT>(),
                opcodes::JMP => self.op_jmp(),
                opcodes::JMP2 => self.op_jmp2(),
                opcodes::JNZ => self.op_jnz::<0x00>(),
                opcodes::JNZ2 => self.op_jnz::<FLAG_SHORT>(),
                opcodes::LDZ => self.op_ldz::<0x00>(),
                opcodes::LDZ2 => self.op_ldz::<FLAG_SHORT>(),
                opcodes::STZ => self.op_stz::<0x00>(),
                opcodes::STZ2 => self.op_stz::<FLAG_SHORT>(),
                opcodes::DEI => self.op_dei(machine),
                opcodes::DEO => self.op_deo(machine),
                opcodes::DEO2 => self.op_deo2(machine),
                opcodes::ADD => self.op_add::<0x00>(),
                opcodes::ADD2 => self.op_add::<FLAG_SHORT>(),
                opcodes::SUB => self.op_sub::<0x00>(),
                opcodes::SUB2 => self.op_sub::<FLAG_SHORT>(),
                opcodes::MUL => self.op_mul::<0x00>(),
                opcodes::MUL2 => self.op_mul::<FLAG_SHORT>(),
                opcodes::DIV => self.op_div::<0x00>(),
                opcodes::DIV2 => self.op_div::<FLAG_SHORT>(),
                opcodes::PUSH => self.op_push(),
                opcodes::PUSH2 => self.op_push2(),
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

    /// Returns a byte of memory
    #[inline]
    pub fn ram_peek_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// Returns a short from memory
    #[inline]
    pub fn ram_peek_short(&self, addr: u16) -> u16 {
        let hi = self.ram[addr as usize];
        let lo = self.ram[addr.wrapping_add(1) as usize];
        u16::from_be_bytes([hi, lo])
    }

    #[inline]
    pub fn ram_poke_byte(&mut self, addr: u16, value: u8) {
        self.ram[addr as usize] = value;
    }

    #[inline]
    pub fn ram_poke_short(&mut self, addr: u16, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        self.ram[addr as usize] = hi;
        self.ram[addr.wrapping_add(1) as usize] = lo;
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
    fn read_short(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize];
        let lo = self.ram[self.pc.wrapping_add(1) as usize];
        self.pc = self.pc.wrapping_add(2);

        u16::from_be_bytes([hi, lo])
    }

    #[inline]
    fn op_inc(&mut self) {
        let value = self.stack.pop_byte();
        self.stack.push_byte(value.wrapping_add(1));
    }

    #[inline]
    fn op_dup(&mut self) {
        let value = self.stack.pop_byte();
        self.stack.push_byte(value);
        self.stack.push_byte(value);
    }

    #[inline]
    fn op_dup2(&mut self) {
        let value = self.stack.pop_short();
        self.stack.push_short(value);
        self.stack.push_short(value);
    }

    #[inline]
    fn op_equ<const FLAGS: u8>(&mut self) {
        let res = if short_mode(FLAGS) {
            let b = self.stack.pop_short();
            let a = self.stack.pop_short();
            a == b
        } else {
            let b = self.stack.pop_byte();
            let a = self.stack.pop_byte();
            a == b
        };

        self.stack.push_byte(if res { 0x01 } else { 0x00 });
    }

    #[inline]
    fn op_jmp(&mut self) {
        let offset = self.stack.pop_byte();
        self.pc = self.pc.wrapping_add(offset as u16);
    }

    #[inline]
    fn op_jmp2(&mut self) {
        let addr = self.stack.pop_short();
        self.pc = addr;
    }

    #[inline]
    fn op_jnz<const FLAGS: u8>(&mut self) {
        let addr = if short_mode(FLAGS) {
            self.stack.pop_short()
        } else {
            let offset = self.stack.pop_byte();
            self.pc.wrapping_add(offset as u16)
        };

        let condition = self.stack.pop_byte();
        if condition != 0x00 {
            self.pc = addr;
        }
    }

    #[inline]
    fn op_ldz<const FLAGS: u8>(&mut self) {
        let addr = self.stack.pop_byte();
        if short_mode(FLAGS) {
            let value = self.ram_peek_short(addr as u16);
            self.stack.push_short(value);
        } else {
            let value = self.ram_peek_byte(addr as u16);
            self.stack.push_byte(value);
        }
    }

    #[inline]
    fn op_stz<const FLAGS: u8>(&mut self) {
        let addr = self.stack.pop_byte();
        if short_mode(FLAGS) {
            let value = self.stack.pop_short();
            self.ram_poke_short(addr as u16, value);
        } else {
            let value = self.stack.pop_byte();
            self.ram_poke_byte(addr as u16, value);
        }
    }

    #[inline]
    fn op_push(&mut self) {
        let value = self.read_byte();
        self.stack.push_byte(value);
    }

    #[inline]
    fn op_push2(&mut self) {
        let value = self.read_short();
        self.stack.push_short(value);
    }

    #[inline]
    fn op_dei(&mut self, machine: &mut impl Machine) {
        let target = self.stack.pop_byte();
        self.stack.push_byte(self.devices[target as usize]);

        // callback for I/O
        machine.dei(self, target);
    }

    #[inline]
    fn op_deo(&mut self, machine: &mut impl Machine) {
        let target = self.stack.pop_byte();

        // write value to device port
        let value = self.stack.pop_byte();
        self.devices[target as usize] = value;

        // callback for I/O
        machine.deo(self, target);
    }

    #[inline]
    fn op_deo2(&mut self, machine: &mut impl Machine) {
        let target = self.stack.pop_byte();

        // write short value to device port
        let value = self.stack.pop_short();
        let [hi, lo] = value.to_be_bytes();
        self.devices[target as usize] = hi;
        self.devices[target.wrapping_add(1) as usize] = lo;

        // callback for I/0
        machine.deo(self, target);
    }

    #[inline]
    fn op_add<const FLAGS: u8>(&mut self) {
        binary_op!(self, FLAGS, |a, b| a.wrapping_add(b))
    }

    #[inline]
    fn op_sub<const FLAGS: u8>(&mut self) {
        binary_op!(self, FLAGS, |a, b| a.wrapping_sub(b))
    }

    #[inline]
    fn op_mul<const FLAGS: u8>(&mut self) {
        binary_op!(self, FLAGS, |a, b| a.wrapping_mul(b))
    }

    #[inline]
    fn op_div<const FLAGS: u8>(&mut self) {
        binary_op!(self, FLAGS, |a, b| a.wrapping_div(b))
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
    use super::opcodes::*;
    use super::*;

    pub struct AnyMachine {}
    impl Machine for AnyMachine {
        fn deo(&mut self, _: &mut Cpu, _: u8) {}
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
        let rom = rom_from(&[0x01, 0x01, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x103);
        assert_eq!(pc, cpu.pc);
    }

    #[test]
    fn run_wraps_pc_at_the_end_of_ram() {
        let mut rom = zeroed_memory();
        rom[rom.len() - 1] = 0x01;
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0xffff, &mut AnyMachine {});

        assert_eq!(pc, 0x01);
        assert_eq!(pc, cpu.pc);
    }

    #[test]
    fn inc_opcode() {
        let rom = rom_from(&[PUSH, 0xff, INC, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0x00);
    }

    #[test]
    fn dup_opcode() {
        let rom = rom_from(&[PUSH, 0xab, DUP, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.byte_at(0), 0xab);
        assert_eq!(cpu.stack.byte_at(1), 0xab);
    }

    #[test]
    fn dup2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcb, DUP2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x105);
        assert_eq!(cpu.stack.len(), 4);
        assert_eq!(cpu.stack.short_at(0), 0xabcb);
        assert_eq!(cpu.stack.short_at(2), 0xabcb);
    }

    #[test]
    fn push_opcode() {
        let rom = rom_from(&[PUSH, 0xab, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x103);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xab);
    }

    #[test]
    fn push2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0xabcd);
    }

    #[test]
    fn dei_opcode() {
        let rom = rom_from(&[PUSH, 0x10, DEI, BRK]);
        let mut cpu = Cpu::new(&rom);
        cpu.devices[0x10] = 0xab;

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xab);
    }

    #[test]
    fn deo_opcode() {
        let rom = rom_from(&[PUSH, 0xab, PUSH, 0x02, DEO, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 0);
        assert_eq!(cpu.devices[0x02], 0xab);
        // TODO: check AnyMachine.deo has been called with 0xab as target arg
    }

    #[test]
    fn deo2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, PUSH, 0x00, DEO2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x107);
        assert_eq!(cpu.stack.len(), 0);
        assert_eq!(cpu.devices[0x00], 0xab);
        assert_eq!(cpu.devices[0x01], 0xcd);
        // TODO: check AnyMachine.deo has been called with 0xab as target arg
    }

    #[test]
    fn add_opcode() {
        let rom = rom_from(&[PUSH, 0xab, PUSH, 0x02, ADD, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xad);
    }

    #[test]
    fn add2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, PUSH2, 0x11, 0x11, ADD2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x108);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0xbcde);
    }

    #[test]
    fn sub_opcode() {
        let rom = rom_from(&[PUSH, 0xab, PUSH, 0x02, SUB, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xa9);
    }

    #[test]
    fn sub2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, PUSH2, 0x11, 0x11, SUB2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x108);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0x9abc);
    }

    #[test]
    fn mul_opcode() {
        let rom = rom_from(&[PUSH, 0x03, PUSH, 0x02, MUL, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0x06);
    }

    #[test]
    fn mul2_opcode() {
        let rom = rom_from(&[PUSH2, 0x11, 0x11, PUSH2, 0x00, 0x02, MUL2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x108);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0x2222);
    }

    #[test]
    fn div_opcode() {
        let rom = rom_from(&[PUSH, 0x07, PUSH, 0x02, DIV, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0x03);
    }

    #[test]
    fn div2_opcode() {
        let rom = rom_from(&[PUSH2, 0x66, 0x66, PUSH2, 0x22, 0x22, DIV2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x108);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0x03);
    }

    #[test]
    fn jmp_opcode() {
        let rom = rom_from(&[PUSH, 0x01, JMP, BRK, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x105);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn jmp2_opcode() {
        let rom = rom_from(&[BRK, PUSH2, 0x01, 0x00, JMP2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x101, &mut AnyMachine {});

        assert_eq!(pc, 0x101);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn jnz_opcode() {
        let rom = rom_from(&[PUSH2, 0x01, 0x01, JNZ, BRK, PUSH2, 0x00, 0x01, JNZ, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x10a);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn jnz2_opcode() {
        let rom = rom_from(&[
            PUSH, 0x01, PUSH2, 0x01, 0x07, JNZ2, BRK, PUSH, 0x00, PUSH2, 0x01, 0x00, JNZ2, BRK,
        ]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x10e);
        assert_eq!(cpu.stack.len(), 0);
    }

    #[test]
    fn ldz_opcode() {
        let rom = rom_from(&[PUSH, 0x01, LDZ, BRK]);
        let mut cpu = Cpu::new(&rom);
        cpu.ram[0x01] = 0xab;

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0xab);
    }

    #[test]
    fn ldz2_opcode() {
        let rom = rom_from(&[PUSH, 0x01, LDZ2, BRK]);
        let mut cpu = Cpu::new(&rom);
        cpu.ram[0x01] = 0xab;
        cpu.ram[0x02] = 0xcd;

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x104);
        assert_eq!(cpu.stack.len(), 2);
        assert_eq!(cpu.stack.short_at(0), 0xabcd);
    }

    #[test]
    fn stz_opcode() {
        let rom = rom_from(&[PUSH, 0xab, PUSH, 0x01, STZ, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x106);
        assert_eq!(cpu.stack.len(), 0);
        assert_eq!(cpu.ram_peek_byte(0x01), 0xab);
    }

    #[test]
    fn stz2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, PUSH, 0x01, STZ2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x107);
        assert_eq!(cpu.stack.len(), 0);
        assert_eq!(cpu.ram_peek_short(0x01), 0xabcd);
    }

    #[test]
    fn equ_opcode() {
        let rom = rom_from(&[PUSH, 0xab, PUSH, 0xab, EQU, PUSH, 0x00, EQU, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x109);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0x00);
    }

    #[test]
    fn equ2_opcode() {
        let rom = rom_from(&[PUSH2, 0xab, 0xcd, PUSH2, 0xab, 0xcd, EQU2, BRK]);
        let mut cpu = Cpu::new(&rom);

        let pc = cpu.run(0x100, &mut AnyMachine {});

        assert_eq!(pc, 0x108);
        assert_eq!(cpu.stack.len(), 1);
        assert_eq!(cpu.stack.byte_at(0), 0x01);
    }
}
