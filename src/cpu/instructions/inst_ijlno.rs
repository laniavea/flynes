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
        self.stack_push_16b(self.program_counter.wrapping_add(2));
        self.program_counter = data_ref;
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

#[test]
fn test_ijlno_operations() {
    let mut cpu = Cpu::default();

    cpu.op_lda_im(0b0101_0101);
    cpu.op_ora_im(0b1010_1010);
    assert_eq!(cpu.reg_a, 0b1111_1111);
    assert!(cpu.get_flag(7));

    cpu.op_lda(0xFFFF);
    cpu.op_ora(0xFFFF);
    assert_eq!(cpu.reg_a, 0b0000_0000);
    assert!(cpu.get_flag(1));

    cpu.op_lda_im(0b0000_0001);
    cpu.op_lsr_a();
    assert_eq!(cpu.reg_a, 0);
    assert!(cpu.get_flag(0));

    cpu.op_ldx_im(0x10);
    cpu.op_ldy_im(0x11);
    assert!(cpu.reg_x == cpu.reg_y - 1);

    cpu.op_jmp(0xFFFF);
    assert_eq!(cpu.program_counter, 0xFFFF);

    cpu.op_jsr(0x1111);
    assert_eq!(cpu.program_counter, 0x1111);
    assert_eq!(cpu.stack_pop_16b(), 0x01);

    cpu.op_inx();
    cpu.op_iny();

    assert_eq!((cpu.reg_x, cpu.reg_y), (0x11, 0x12));
    cpu.op_inc(0xFFFF);
    assert_eq!(cpu.read_mem(0xFFFF), 1);
}
