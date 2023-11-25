use crate::TD4::Register::*;
use crate::TD4::Rom::*;
use crate::TD4::Port::*;
use crate::TD4::OpCode::*;

pub struct CPU {
    register: Register,
    port: Port,
    rom: Rom
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> Self {
        CPU { register: Register::new(), port: Port::new(), rom: Rom::new(rom) }
    }

    pub fn tick(&mut self) {
        let (next_order, im) = self.fetch().unwrap();
        println!("{:?}, {:04b}", next_order, im);
        self.execute(next_order, im);

        println!("{}", self.port);
        println!("{}", self.register);
    }

    fn fetch(&mut self) -> Option<(OpCode, u8)> {
        let pc = self.register.pc;
        let rom_value = self.rom.mem_array[pc as usize];

        let op = rom_value >> 4; // 後ろ 4 bit
        let im = rom_value & 0b00001111; // 前 4 bit

        match op {
            0b0000 => Some((OpCode::AddA, im)),
            0b0101 => Some((OpCode::AddB, im)),
            0b0011 => Some((OpCode::MovA, im)),
            0b0111 => Some((OpCode::MovB, im)),
            0b0001 => Some((OpCode::MovAB, im)),
            0b0100 => Some((OpCode::MovBA, im)),
            0b1111 => Some((OpCode::Jmp, im)),
            0b1110 => Some((OpCode::Jnc, im)),
            0b0010 => Some((OpCode::InA, im)),
            0b0110 => Some((OpCode::InB, im)),
            0b1001 => Some((OpCode::OutB, im)),
            0b1011 => Some((OpCode::OutIm, im)),
            _ => None
        }
    }

    fn execute(&mut self, op: OpCode, im: u8) {
        match op {
            OpCode::AddA => {
                self.register.carry = if (self.register.reg_a + im) >> 4 != 0 { true } else { false };
                self.register.reg_a = 0b00001111 & (self.register.reg_a + im);
                self.register.inc_pc();
            },
            OpCode::AddB => {
                self.register.carry = if (self.register.reg_b + im) >> 4 != 0 { true } else { false };
                self.register.reg_b = 0b00001111 & (self.register.reg_b + im);
                self.register.inc_pc();
            },
            OpCode::MovA => {
                self.register.reg_a = im;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::MovB => {
                self.register.reg_b = im;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::MovAB => {
                self.register.reg_a = self.register.reg_b;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::MovBA => {
                self.register.reg_b = self.register.reg_a;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::Jmp => {
                self.register.pc = im;
                self.register.carry = false;
            },
            OpCode::Jnc => {
                if !self.register.carry {
                    self.register.pc = im;
                } else {
                    self.register.pc += 1;
                }
                self.register.carry = false;
            },
            OpCode::InA => {
                self.register.reg_a = self.port.input;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::InB => {
                self.register.reg_b = self.port.input;
                self.register.carry = false;
                self.register.inc_pc();
            }
            OpCode::OutB => {
                self.port.output = self.register.reg_b;
                self.register.carry = false;
                self.register.inc_pc();
            },
            OpCode::OutIm => {
                self.port.output = im;
                self.register.carry = false;
                self.register.inc_pc();
            },
        }
    }
}
