use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_lda(&mut self, data_ref: u16) {
        self.reg_a = self.memory[data_ref as usize];
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_lda_im(&mut self, data: u16) {
        self.reg_a = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_ldx(&mut self, data_ref: u16) {
        self.reg_x = self.memory[data_ref as usize];
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_ldx_im(&mut self, data: u16) {
        self.reg_x = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_ldy(&mut self, data_ref: u16) {
        self.reg_y = self.memory[data_ref as usize];
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_ldy_im(&mut self, data: u16) {
        self.reg_y = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_lsr(&mut self, data_ref: u16) {
        let now_data = &mut self.memory[data_ref as usize];
        self.cpu_status = update_carry_flag(self.cpu_status, *now_data);
        *now_data >>= 1;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, *now_data)
    }

    pub fn op_lsr_a(&mut self) {
        self.cpu_status = update_carry_flag(self.cpu_status, self.reg_a);
        self.reg_a >>= 1;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a)
    }
}
