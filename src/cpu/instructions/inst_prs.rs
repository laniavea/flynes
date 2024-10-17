use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

// All obelisk 6502 instructions which starts with P, R or S.
// Instructions here: SBC, SEC, SED, SEI, STA, STX, STY
// More info: https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
// TODO: Compete PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS
impl Cpu {
    pub fn op_sbc(&mut self, data_ref: u16) {
        let data = self.memory[data_ref as usize];
        // Formula is A(ccumulator) - M(emory) - (C(arry) - 1)
        let temp_val = if get_flag(self.cpu_status, 0) {
            self.reg_a.wrapping_sub(data)
        } else {
            self.reg_a.wrapping_sub(data).wrapping_sub(1)
        };

        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        if ((self.reg_a^temp_val) & ((data)^temp_val).wrapping_neg()) & 0b1000_0000 == 0b1000_0000 {
            self.cpu_status = set_overflow_flag(self.cpu_status, true);
        } else {
            self.cpu_status = set_overflow_flag(self.cpu_status, false);
        }

        // Sets because it's sum of reg_a and inv data -> if result < reg_a -> reg_a + data > 255
        if temp_val <= self.reg_a {
            self.cpu_status = set_carry_flag(self.cpu_status, true);
        } else {
            self.cpu_status = set_carry_flag(self.cpu_status, false);
        }

        self.reg_a = temp_val;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_sbc_im(&mut self, data: u16) {
        // Formula is A(ccumulator) - M(emory) - (C(arry) - 1)
        let temp_val = if get_flag(self.cpu_status, 0) {
            self.reg_a.wrapping_sub(data as u8)
        } else {
            self.reg_a.wrapping_sub(data as u8).wrapping_sub(1)
        };

        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        if ((self.reg_a^temp_val) & ((data as u8)^temp_val).wrapping_neg()) & 0b1000_0000 == 0b1000_0000 {
            self.cpu_status = set_overflow_flag(self.cpu_status, true);
        } else {
            self.cpu_status = set_overflow_flag(self.cpu_status, false);
        }

        // Sets because it's sum of reg_a and inv data -> if result < reg_a -> reg_a + data > 255
        if temp_val <= self.reg_a {
            self.cpu_status = set_carry_flag(self.cpu_status, true);
        } else {
            self.cpu_status = set_carry_flag(self.cpu_status, false);
        }

        self.reg_a = temp_val;
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
    assert_eq!(cpu.cpu_status, 0b0000_1101);
    cpu.op_sei();
    cpu.op_sec();
    cpu.op_sed();
    assert_eq!(cpu.cpu_status, 0b0000_1101);
    
    // Tested on https://skilldrick.github.io/easy6502/
    // Assemble
    // LDA #$00
    // SBC #$64
    // SBC #$9B
    // SBC #$01
    // SBC #$FC
    // SBC #$02
    // SBC #$80
    // SBC #$80
    cpu.cpu_status = 0b0000_0000;
    cpu.op_lda_im(0);
    cpu.op_sbc_im(0x64);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x9B,0b1000_0000));
    cpu.op_sbc_im(0x9B);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0xFF, 0b1000_0000));
    cpu.op_sbc_im(0x01);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0xFD, 0b1000_0001));
    cpu.op_sbc_im(0xFC);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x01, 0b0000_0001));
    cpu.op_sbc_im(0x02);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0xFF, 0b1000_0000));
    cpu.op_sbc_im(0x80);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x7E, 0b0000_0001));
    cpu.op_sbc_im(0x80);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0xFE, 0b1100_0000));

    //TODO: write all tests
}
