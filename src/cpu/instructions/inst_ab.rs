use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

use crate::cpu::{OVERFLOW_FLAG, BREAK_FLAG, NEGATIVE_FLAG, ZERO_FLAG};

impl Cpu {
    pub fn op_bit(&mut self, data: u16) {
        let result = self.reg_a & (data as u8);

        if result == 0 { self.set_flag(ZERO_FLAG, true); }
        self.set_flag(NEGATIVE_FLAG, get_flag_inl(result, NEGATIVE_FLAG));
        self.set_flag(OVERFLOW_FLAG, get_flag_inl(result, OVERFLOW_FLAG));
    }

    pub fn op_bmi(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(NEGATIVE_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bne(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(ZERO_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bpl(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(NEGATIVE_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_brk(&mut self) {
        self.stack_push_16b(self.program_counter);
        self.stack_push(self.cpu_status);

        self.set_flag(BREAK_FLAG, true);
        self.program_counter = self.read_mem_16b(0xFFFE);

    }

    pub fn op_bvc(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(OVERFLOW_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bvs(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(OVERFLOW_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }
}
