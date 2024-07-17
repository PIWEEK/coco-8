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
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.index as usize
    }

    pub fn push_byte(&mut self, x: u8) {
        self.data[self.index as usize] = x;
        self.index = self.index.wrapping_add(1);
    }

    pub fn byte_at(&self, i: u8) -> u8 {
        return self.data[i as usize];
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.index.saturating_sub(8)..self.index {
            write!(f, "{:02x}", self.byte_at(i))?;
            if i < self.index - 1 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}
