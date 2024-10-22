use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_inc(&mut self, data_ref: u16) {
        let now_data = self.read_mem(data_ref).wrapping_add(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, now_data);
        self.write_mem(data_ref, now_data);
    }

    pub fn op_inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_iny(&mut self) {
        self.reg_y = self.reg_y.wrapping_add(1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_jmp(&mut self, data_ref: u16) {
        self.program_counter = data_ref;
    }

    pub fn op_jsr(&mut self, data_ref: u16) {
        self.stack_push_16b(data_ref.wrapping_add(1));
        self.program_counter = self.read_mem_16b(data_ref);
    }

    pub fn op_lda(&mut self, data_ref: u16) {
        self.reg_a = self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_lda_im(&mut self, data: u16) {
        self.reg_a = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_ldx(&mut self, data_ref: u16) {
        self.reg_x = self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_ldx_im(&mut self, data: u16) {
        self.reg_x = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_ldy(&mut self, data_ref: u16) {
        self.reg_y = self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_ldy_im(&mut self, data: u16) {
        self.reg_y = data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_lsr(&mut self, data_ref: u16) {
        let now_data = self.read_mem(data_ref);
        self.cpu_status = update_carry_flag(self.cpu_status, now_data);
        self.write_mem(data_ref, now_data >> 1);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, now_data >> 1)
    }

    pub fn op_lsr_a(&mut self) {
        self.cpu_status = update_carry_flag(self.cpu_status, self.reg_a);
        self.reg_a >>= 1;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a)
    }

    // No operation, instead changes to program_counter
    pub fn op_nop(&self) { }

    pub fn op_ora(&mut self, data_ref: u16) {
        self.reg_a |= self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_ora_im(&mut self, data: u16) {
        self.reg_a |= data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }
}

//TODO: Write tests
