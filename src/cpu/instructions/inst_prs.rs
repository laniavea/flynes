use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

// All obelisk 6502 instructions which starts with P, R or S.
// Instructions here: SBC, SEC, SED, SEI, STA, STX, STY
// More info: https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
// TODO: Compete PHA, PHP, PLA, PLP, ROL, ROR, RTI, RYS, SBC
impl Cpu {
    pub fn op_sbc(&mut self, data_ref: u16) {
        //TODO: Complete SBC instruction

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_sbc_im(&mut self, data: u16) {
        //TODO: Complete SBC Im instruction
        let new_data = if get_flag(self.cpu_status, 0) {
            (data as u8).wrapping_sub(2)
        } else {
            (data as u8).wrapping_sub(1)
        };

        let temp_val = self.reg_a.wrapping_sub(new_data as u8);
        if (self.reg_a^temp_val & (self.reg_a^(data as u8)).wrapping_neg()) & 0b1000_0000 == 0b1000_0000 {
            self.cpu_status = set_overflow_flag(self.cpu_status, true);
        } else {
            self.cpu_status = set_overflow_flag(self.cpu_status, false);
        }

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
        self.reg_a = temp_val
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
    assert_eq!(cpu.cpu_status, 0b0000_1101);
    cpu.op_sei();
    cpu.op_sec();
    cpu.op_sed();
    assert_eq!(cpu.cpu_status, 0b0000_1101);
    
    // Assemble
    // LDA #$00
    // SBC #$64
    // SBC #$9B
    // SBC #$01
    // SBC #$FC
    // SBC #$02
    cpu.reg_a = 0;
    cpu.cpu_status = 0b0000_0000;
    cpu.op_sbc_im(0x64);
    assert_eq!(cpu.reg_a, 0x9B);
    cpu.op_sbc_im(0x9B);
    assert_eq!(cpu.reg_a, 0xFF);
    cpu.op_sbc_im(0x01);
    assert_eq!(cpu.reg_a, 0xFD);
    cpu.op_sbc_im(0xFC);
    assert_eq!(cpu.reg_a, 0x01);
    cpu.op_sbc_im(0x02);
    assert_eq!(cpu.reg_a, 0xFF);
    //TODO: write SBC and higher tests
}
