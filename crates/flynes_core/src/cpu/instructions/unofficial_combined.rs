use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::{CARRY_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, set_flag, is_flag_set};

impl Cpu {
    /// ALR / ASR operation; AND byte with reg_a, then shift right one bit in reg_a
    pub fn op_alr(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a &= self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a & 0b0000_0001 == 0b0000_0001);
        self.reg_a >>= 1;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// ANC / AAC operation; AND byte with reg_a, then copies N (bit 7) to carry flag
    pub fn op_anc(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a &= self.read_8bit(bus, data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
        let is_neg_set = is_flag_set(&self.cpu_status, NEGATIVE_FLAG);
        set_flag(&mut self.cpu_status, CARRY_FLAG, is_neg_set);
    }

    /// ARR operation; Simular to AND then ROR, but C is bit 6, and V is xor bit 6 and bit 5
    pub fn op_arr(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a &= self.read_8bit(bus, data_ref);
        self.reg_a >>= 1;
        if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            self.reg_a |= 0b1000_0000;
        }

        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
        let is_carry_flag_set = self.reg_a & 0b0100_0000 == 0b0100_0000; 
        set_flag(&mut self.cpu_status, CARRY_FLAG, is_carry_flag_set);
        let is_overflow_flag_set = (self.reg_a & 0b0010_0000 == 0b0010_0000) ^ is_carry_flag_set;
        set_flag(&mut self.cpu_status, OVERFLOW_FLAG, is_overflow_flag_set);
    }

    /// AXS / SBX / SAX operation; Sets reg_x to ((reg_x AND reg_a) - value) and updates Z, N and C
    pub fn op_axs(&mut self, bus: &mut Bus, data_ref: u16) {
        let temp_and_result = self.reg_x & self.reg_a;
        let read_data = self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, temp_and_result >= read_data);
        self.reg_x = temp_and_result.wrapping_sub(read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x);
    }

    /// LAX operation; Same as LDA then TAX
    pub fn op_lax(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a = self.read_8bit(bus, data_ref);
        self.reg_x = self.reg_a;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x)
    }

    /// SAX / AAX / AXS operation; Stores bitwise AND for reg_a and reg_x
    pub fn op_sax(&mut self, bus: &mut Bus, data_ref: u16) {
        self.write_8bit(bus, data_ref, self.reg_a & self.reg_x);
    }
}
