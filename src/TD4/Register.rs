use core::fmt;

#[derive(Debug)]
pub struct Register {
    pub reg_a: u8,
    pub reg_b: u8,
    pub carry: bool,
    pub pc: u8,
}

impl Register {
    pub fn new() -> Self {
        Register { reg_a: 0, reg_b: 0, carry: false, pc: 0 }
    }

    pub fn inc_pc(&mut self) {
        self.pc += 0b0001;
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Register {{ reg_a: {:04b}, reg_b: {:04b}, carry: {}, pc: {:04b} }}", self.reg_a, self.reg_b, self.carry, self.pc)
    }
}

