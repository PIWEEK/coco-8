pub const BRK: u8 = 0x00;
pub const INC: u8 = 0x01;
pub const DUP: u8 = 0x06;
pub const DUP2: u8 = 0x26;
pub const JMP: u8 = 0x0c;
pub const JMP2: u8 = 0x2c;
pub const JNZ: u8 = 0x0d;
pub const JNZ2: u8 = 0x2d;
pub const LDZ: u8 = 0x10;
pub const LDZ2: u8 = 0x30;
pub const STZ: u8 = 0x11;
pub const STZ2: u8 = 0x31;
pub const DEI: u8 = 0x16;
pub const DEO: u8 = 0x17;
pub const DEO2: u8 = 0x37;
pub const ADD: u8 = 0x18;
pub const ADD2: u8 = 0x38;
pub const SUB: u8 = 0x19;
pub const SUB2: u8 = 0x39;
pub const MUL: u8 = 0x1a;
pub const MUL2: u8 = 0x3a;
pub const DIV: u8 = 0x1b;
pub const DIV2: u8 = 0x3b;
pub const PUSH: u8 = 0x80;
pub const PUSH2: u8 = 0xa0;

pub const FLAG_SHORT: u8 = 0b0010_0000;
pub const FLAG_RET: u8 = 0b0100_0000;
pub const FLAG_KEEP: u8 = 0b1000_0000;

pub fn short_mode(opcode: u8) -> bool {
    (opcode & FLAG_SHORT) == FLAG_SHORT
}

pub fn ret_mode(opcode: u8) -> bool {
    (opcode & FLAG_RET) == FLAG_RET
}

pub fn keep_mode(opcode: u8) -> bool {
    (opcode & FLAG_KEEP) == FLAG_KEEP
}
