use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_clc(&mut self) {
        self.cpu_status = set_carry_flag(self.cpu_status, false);
    }

    pub fn op_cld(&mut self) {
        self.cpu_status = set_decimal_flag(self.cpu_status, false);
    }

    pub fn op_cli(&mut self) {
        self.cpu_status = set_interrupt_flag(self.cpu_status, false);
    }

    pub fn op_clv(&mut self) {
        self.cpu_status = set_overflow_flag(self.cpu_status, false);
    }

    pub fn op_cmp(&mut self, data_ref: u16) {
        self.compare_it(self.reg_a, self.read_mem(data_ref));
    }

    pub fn op_cmp_im(&mut self, data: u16) {
        self.compare_it(self.reg_a, data as u8);
    }

    pub fn op_cpx(&mut self, data_ref: u16) {
        self.compare_it(self.reg_x, self.read_mem(data_ref));
    }

    pub fn op_cpx_im(&mut self, data: u16) {
        self.compare_it(self.reg_x, data as u8);
    }

    pub fn op_cpy(&mut self, data_ref: u16) {
        self.compare_it(self.reg_y, self.read_mem(data_ref));
    }

    pub fn op_cpy_im(&mut self, data: u16) {
        self.compare_it(self.reg_y, data as u8);
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

    #[inline]
    fn compare_it(&mut self, regiser_data: u8, data_to_comp: u8) {
        match regiser_data.cmp(&data_to_comp) {
            std::cmp::Ordering::Greater => {
                self.cpu_status = set_carry_flag(self.cpu_status, true);
                self.cpu_status = set_negative_flag(self.cpu_status, regiser_data.wrapping_sub(data_to_comp) >= 128)
            },
            std::cmp::Ordering::Less => {
                self.cpu_status = set_negative_flag(self.cpu_status, regiser_data.wrapping_sub(data_to_comp) >= 128)
            },
            std::cmp::Ordering::Equal => {
                self.cpu_status = set_carry_flag(self.cpu_status, true);
                self.cpu_status = set_zero_flag(self.cpu_status, true);
            }
        }
    }
}

