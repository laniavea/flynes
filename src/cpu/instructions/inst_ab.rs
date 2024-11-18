use crate::cpu::Cpu;
use crate::cpu::{OVERFLOW_FLAG, BREAK_FLAG, NEGATIVE_FLAG, ZERO_FLAG, CARRY_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, set_carry_flag, get_flag_inl, set_overflow_flag};

impl Cpu {
    pub fn op_adc(&mut self, data_ref: u16) {
        let data = self.read_mem(data_ref);
        let temp_val = if get_flag_inl(self.cpu_status, CARRY_FLAG) {
            self.reg_a.wrapping_add(data).wrapping_add(1)
        } else {
            self.reg_a.wrapping_add(data)
        };

        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        let over_fl_st = ((self.reg_a^temp_val) & (data^temp_val)) & 0b1000_0000 == 0b1000_0000;
        self.cpu_status = set_overflow_flag(self.cpu_status, over_fl_st);

        // Sets because it's sum of reg_a and data -> if result < reg_a -> reg_a + data > 255
        self.cpu_status = set_carry_flag(self.cpu_status, temp_val <= self.reg_a);

        self.reg_a = temp_val;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_adc_im(&mut self, data: u16) {
        let temp_val = if get_flag_inl(self.cpu_status, CARRY_FLAG) {
            self.reg_a.wrapping_add(data as u8).wrapping_add(1)
        } else {
            self.reg_a.wrapping_add(data as u8)
        };

        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        let over_fl_st = ((self.reg_a^temp_val) & ((data as u8)^temp_val)) & 0b1000_0000 == 0b1000_0000;
        self.cpu_status = set_overflow_flag(self.cpu_status, over_fl_st);

        // Sets because it's sum of reg_a and data -> if result < reg_a -> reg_a + data > 255
        self.cpu_status = set_carry_flag(self.cpu_status, temp_val <= self.reg_a);

        self.reg_a = temp_val;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_and(&mut self, data_ref: u16) {
        self.reg_a &= self.read_mem(data_ref);
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a)
    }

    pub fn op_and_im(&mut self, data: u16) {
        self.reg_a &= data as u8;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a)
    }

    pub fn op_asl(&mut self, data_ref: u16) {
        let data = self.read_mem(data_ref);

        self.cpu_status = set_carry_flag(self.cpu_status, data >= 128);
        self.reg_a = data << 1;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_asl_acc(&mut self) {
        self.cpu_status = set_carry_flag(self.cpu_status, self.reg_a >= 128);
        self.reg_a <<= 1;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_bcc(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(CARRY_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bcs(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(CARRY_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_beq(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(ZERO_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bit(&mut self, data_ref: u16) {
        let result = self.reg_a & self.read_mem(data_ref);

        if result == 0 { self.set_flag(ZERO_FLAG, true); }
        self.set_flag(NEGATIVE_FLAG, get_flag_inl(result, NEGATIVE_FLAG));
        self.set_flag(OVERFLOW_FLAG, get_flag_inl(result, OVERFLOW_FLAG));
    }

    pub fn op_bmi(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(NEGATIVE_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bne(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(ZERO_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bpl(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(NEGATIVE_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_brk(&mut self) {
        self.stack_push_16b(self.program_counter);
        self.stack_push(self.cpu_status);

        self.set_flag(BREAK_FLAG, true);
        self.program_counter = self.read_mem_16b(0xFFFE);

    }

    pub fn op_bvc(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if !self.get_flag(OVERFLOW_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }

    pub fn op_bvs(&mut self, relative_displacemnt: u16) {
        let relative_displacemnt = relative_displacemnt as u8;
        if self.get_flag(OVERFLOW_FLAG) {
            self.program_counter = self.program_counter.wrapping_add((relative_displacemnt as i8) as u16);
        }
    }
}

#[test]
fn test_ab_operations() {
    let mut cpu = Cpu {
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    // Tested on https://skilldrick.github.io/easy6502/
    //
    // Assembly:
    // LDA #$90
    // ADC #$90
    // ADC #$20
    // ADC #$50
    // ADC #$6F
    // ADC #$FF
    // ADC #$7F
    // ADC #$80

    cpu.op_lda_im(0x90);
    cpu.op_adc_im(0x90);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x20, 0b0100_0001));
    cpu.op_adc_im(0x20);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x41, 0b0000_0000));
    cpu.op_adc_im(0x50);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x91, 0b1100_0000));
    cpu.op_adc_im(0x6F);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x00, 0b0000_0011));
    cpu.op_adc_im(0xFF);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x00, 0b0000_0011));
    cpu.op_adc_im(0x7F);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x80, 0b1100_0000));
    cpu.op_adc_im(0x80);
    assert_eq!((cpu.reg_a, cpu.cpu_status), (0x00, 0b0100_0011));
    

}
