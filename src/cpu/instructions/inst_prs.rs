use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

impl Cpu {
    pub fn op_sbc(&mut self, data_ref: u16) {
        //TODO: Complete SBC instruction

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_sbc_im(&mut self, data: u16) {
        //TODO: Complete SBC Im instruction
        let temp_val = self.reg_a;

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

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

#[test]
fn test_prs_operations() {
    let mut cpu = Cpu::new();

    cpu.reg_x = 1;
    cpu.reg_y = 127;
    cpu.reg_a = 255;

    cpu.op_sty(0xFFA0);
    cpu.op_stx(0xF000);
    cpu.op_sta(0x0F00);

    assert_eq!((cpu.memory[0xFFA0], cpu.memory[0xF000], cpu.memory[0x0F00]), (127, 1, 255));

    cpu.cpu_status = 0;
    cpu.op_sei();
    cpu.op_sec();
    cpu.op_sed();
    assert_eq!(cpu.cpu_status, 0b00001101);
    cpu.op_sei();
    cpu.op_sec();
    cpu.op_sed();
    assert_eq!(cpu.cpu_status, 0b00001101);
}
