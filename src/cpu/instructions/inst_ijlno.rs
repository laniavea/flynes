use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

impl Cpu {
    pub fn op_lda(&mut self, set_a: u8) {
        self.reg_a = set_a;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, set_a)
    }
}

