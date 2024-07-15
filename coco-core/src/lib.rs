#![no_std]
#![forbid(unsafe_code)]

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
    pub fn new(ram: &'a mut [u8; 0x10000]) -> Self {
        Self {
            ram,
            stack: Stack::new(),
            ret_stack: Stack::new(),
            pc: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_cpu() {
        let mut memory = [0_u8; 0x10000];
        let cpu = Cpu::new(&mut memory);

        assert_eq!(cpu.pc, 0);
    }
}
