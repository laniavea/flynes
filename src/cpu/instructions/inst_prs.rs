use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

// All obelisk 6502 instructions which starts with P, R or S.
// Instructions here: PHA, PHP, PLA, PLP, ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY
// More info: https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
impl Cpu {
    pub fn op_pha(&mut self) {
        self.stack_push(self.reg_a);
    }

    pub fn op_php(&mut self) {
        self.stack_push(self.cpu_status);
    }

    pub fn op_pla(&mut self) {
        self.reg_a = self.stack_pop();
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_plp(&mut self) {
        self.cpu_status = self.stack_pop();
    }

    pub fn op_rol(&mut self, data_ref: u16) {
        let mut now_mem = self.read_mem(data_ref);
        if get_flag_inl(self.cpu_status, 0) {
            self.cpu_status = update_carry_flag_by_7_bit(self.cpu_status, now_mem);
            now_mem <<= 1;
            now_mem += 0b0000_0001;
        } else {
            self.cpu_status = update_carry_flag_by_7_bit(self.cpu_status, now_mem);
            now_mem <<= 1;
        }
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, now_mem);
        self.write_mem(data_ref, now_mem);
    }

    pub fn op_rol_a(&mut self) {
        if get_flag_inl(self.cpu_status, 0) {
            self.cpu_status = update_carry_flag_by_7_bit(self.cpu_status, self.reg_a);
            self.reg_a <<= 1;
            self.reg_a += 0b0000_0001;
        } else {
            self.cpu_status = update_carry_flag_by_7_bit(self.cpu_status, self.reg_a);
            self.reg_a <<= 1;
        }

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_ror(&mut self, data_ref: u16) {
        let mut now_mem = self.read_mem(data_ref);
        if get_flag_inl(self.cpu_status, 0) {
            self.cpu_status = update_carry_flag(self.cpu_status, now_mem);
            now_mem >>= 1;
            now_mem += 0b1000_0000;
        } else {
            self.cpu_status = update_carry_flag(self.cpu_status, now_mem);
            now_mem >>= 1;
        }
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, now_mem);
        self.write_mem(data_ref, now_mem);
    }

    pub fn op_ror_a(&mut self) {
        if get_flag_inl(self.cpu_status, 0) {
            self.cpu_status = update_carry_flag(self.cpu_status, self.reg_a);
            self.reg_a >>= 1;
            self.reg_a += 0b1000_0000;
        } else {
            self.cpu_status = update_carry_flag(self.cpu_status, self.reg_a);
            self.reg_a >>= 1;
        }

        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_rti(&mut self) {
        self.cpu_status = self.stack_pop();
        self.program_counter = self.stack_pop_16b();
    }

    pub fn op_rts(&mut self) {
        self.program_counter = self.stack_pop_16b().wrapping_add(1);
    }

    pub fn op_sbc(&mut self, data_ref: u16) {
        let data = self.read_mem(data_ref);
        // Formula is A(ccumulator) - M(emory) - (C(arry) - 1)
        let temp_val = if get_flag_inl(self.cpu_status, 0) {
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
        let temp_val = if get_flag_inl(self.cpu_status, 0) {
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
        self.write_mem(data_ref, self.reg_a);
    }

    pub fn op_stx(&mut self, data_ref: u16) {
        self.write_mem(data_ref, self.reg_x);
    }

    pub fn op_sty(&mut self, data_ref: u16) {
        self.write_mem(data_ref, self.reg_y);
    }
}

#[test]
fn test_prs_operations() {
    let mut cpu = Cpu::new();

    // STA, STX, STY instructions
    cpu.reg_x = 1;
    cpu.reg_y = 127;
    cpu.reg_a = 255;

    cpu.op_sty(0xFFA0);
    cpu.op_stx(0xF000);
    cpu.op_sta(0x0F00);

    assert_eq!((cpu.read_mem(0xFFA0), cpu.read_mem(0xF000), cpu.read_mem(0x0F00)), (127, 1, 255));

    // SEI, SEC, SED instructions
    cpu.cpu_status = 0b0000_0000;
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

    // RTS, RTI instructions
    cpu.stack_pointer = 0x0;
    for now_i in 0..=0xFF {
        cpu.stack_push(now_i);
    }

    cpu.op_rts();
    assert_eq!(cpu.program_counter, 0xFF + (0xFE << 8) + 1);

    cpu.op_rti();
    assert_eq!(cpu.cpu_status, 0xFD);
    assert_eq!(cpu.program_counter, 0xFC + (0xFB << 8));

    // Tested on https://skilldrick.github.io/easy6502/
    //
    // LDA #$5D
    // ROR A
    // ROR A
    // ROL A
    // ROL A
    // ROL A
    // ROL A
    cpu.reg_a = 0b0101_1101;
    cpu.set_flag(0, false);
    cpu.op_ror_a();
    assert_eq!(cpu.reg_a, 0b0010_1110);
    assert!(cpu.get_flag(0));
    cpu.op_ror_a();
    assert!(!cpu.get_flag(0));
    assert_eq!(cpu.reg_a, 0b1001_0111);

    cpu.op_rol_a();
    cpu.op_rol_a();
    assert_eq!(cpu.reg_a, 0b0101_1101);
    assert!(!cpu.get_flag(0));
    cpu.op_rol_a();
    cpu.op_rol_a();
    assert!(cpu.get_flag(0));
    assert_eq!(cpu.reg_a, 0b0111_0100);

    // PLP, PLA, PHP, PHA instructions
    cpu.stack_pointer = 0x0;
    for _ in 0..=0xFF {
        cpu.stack_push(0);
    }

    cpu.stack_pointer = 0x10;
    cpu.reg_a = 0x01;
    cpu.cpu_status = 0x02;

    cpu.op_pha();
    cpu.op_php();
    assert_eq!(cpu.read_mem_16b(0x1FF - cpu.stack_pointer as u16 + 1), 0x02 + (0x01 << 8));
    cpu.reg_a = 0;
    cpu.cpu_status = 0;
    cpu.op_pla();
    cpu.op_plp();
    // Reversed because PLA changes cpu_status
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x02, 0x01));
}
