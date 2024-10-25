use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn cpy(&mut self, data_ref: u16) {

    }

    pub fn cpy_im(&mut self, data: u16) {
        match self.reg_y.cmp(&(data as u8)) {
            std::cmp::Ordering::Greater => {

            },
            std::cmp::Ordering::Less => {

            },
            std::cmp::Ordering::Equal => {

            }
        }
    }

    // TODO: RECHECK NEGATIVE FLAG can be wrong
    pub fn op_dec(&mut self, data_ref: u16) {
        let now_data = self.read_mem(data_ref).wrapping_sub(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, now_data);
        self.write_mem(data_ref, now_data);
    }

    pub fn op_dex(&mut self) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_dey(&mut self) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_eor(&mut self, data_ref: u16) {
        self.reg_a ^= self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_eor_im(&mut self, data: u16) {
        self.reg_a ^= data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }
}
