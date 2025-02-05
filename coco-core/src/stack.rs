use core::fmt;

#[derive(Debug)]
pub struct Stack {
    data: [u8; 0x100],
    index: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0_u8; 0x100],
            index: u8::MAX,
        }
    }

    pub fn len(&self) -> usize {
        self.index.wrapping_add(1) as usize
    }

    pub fn push_byte(&mut self, x: u8) {
        self.index = self.index.wrapping_add(1);
        self.data[self.index as usize] = x;
    }

    pub fn push_short(&mut self, x: u16) {
        let [hi, lo] = x.to_be_bytes();
        self.push_byte(hi);
        self.push_byte(lo);
    }

    pub fn pop_byte(&mut self) -> u8 {
        let res = self.data[self.index as usize];
        self.index = self.index.wrapping_sub(1);
        res
    }

    pub fn pop_short(&mut self) -> u16 {
        let lo = self.pop_byte();
        let hi = self.pop_byte();
        u16::from_be_bytes([hi, lo])
    }

    pub fn byte_at(&self, i: u8) -> u8 {
        return self.data[i as usize];
    }

    #[cfg(test)]
    pub fn short_at(&self, i: u8) -> u16 {
        let hi = self.data[i as usize];
        let lo = self.data[i.wrapping_add(1) as usize];

        u16::from_be_bytes([hi, lo])
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.len().saturating_sub(8)..self.len() {
            write!(f, "{:02x}", self.byte_at(i as u8))?;
            if i < self.len() - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}
