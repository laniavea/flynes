use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_tax(&mut self) {
        self.reg_x = self.reg_a;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_tay(&mut self) {
        self.reg_y = self.reg_a;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_tsx(&mut self) {
        self.reg_x = self.stack_pointer;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_txa(&mut self) {
        self.reg_a = self.reg_x;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_txs(&mut self) {
        self.stack_pointer = self.reg_x;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.stack_pointer);
    }

    pub fn op_tya(&mut self) {
        self.reg_a = self.reg_y;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }
}
