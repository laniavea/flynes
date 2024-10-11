use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_sec(&mut self) {
        self.cpu_status = set_carry_flag(self.cpu_status, true)
    }

    pub fn op_sed(&mut self) {
        self.cpu_status = set_decimal_flag(self.cpu_status, true)
    }

    pub fn op_sei(&mut self) {
        self.cpu_status = set_interrupt_flag(self.cpu_status, true)
    }

    pub fn op_sta(&mut self, data_ref: u16) {
        self.memory[data_ref as usize] = self.reg_a;
    }

    pub fn op_stx(&mut self, data_ref: u16) {
        self.memory[data_ref as usize] = self.reg_x;
    }

    pub fn op_sty(&mut self, data_ref: u16) {
        self.memory[data_ref as usize] = self.reg_y;
    }
}
