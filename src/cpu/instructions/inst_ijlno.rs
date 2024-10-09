use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

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
}
