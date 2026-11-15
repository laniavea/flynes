use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::{CARRY_FLAG, OVERFLOW_FLAG};
use crate::cpu::instructions::shared_ops::{set_flag, update_zero_and_neg_flags, is_flag_set};

impl Cpu {
    /// DCP / DCM operations; Subtract 1 from memory (without borrow) then compare
    pub fn op_dcp(&mut self, bus: &mut Bus, data_ref: u16) {
        let read_data = self.read_8bit(bus, data_ref).wrapping_sub(1);
        self.write_8bit(bus, data_ref, read_data);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= read_data);
        let temp_res = self.reg_a.wrapping_sub(read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, temp_res);
    }

    /// ISC / ISB / INS operations; Add 1 from memory (without borrow) then SBC
    pub fn op_isc(&mut self, bus: &mut Bus, data_ref: u16) {
        let read_data = self.read_8bit(bus, data_ref).wrapping_add(1);
        self.write_8bit(bus, data_ref, read_data);

        // SBC
        self.reg_a = if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let temp_val = self.reg_a.wrapping_sub(read_data);
            let over_fl_st =
                (self.reg_a ^ temp_val) & ((0b1111_1111 - read_data) ^ temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val <= self.reg_a);
            temp_val
        } else {
            let temp_val = self.reg_a.wrapping_sub(read_data).wrapping_sub(1);
            let over_fl_st =
                (self.reg_a ^ temp_val) & ((0b1111_1111 - read_data) ^ temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val < self.reg_a);
            temp_val
        };

        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// RLA operations; ROL and then AND;
    pub fn op_rla(&mut self, bus: &mut Bus, data_ref: u16) {
        // ROL
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        let mut read_data = self.read_8bit(bus, data_ref);
        self.write_8bit(bus, data_ref, read_data);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b1000_0000 == 0b1000_0000);
        read_data <<= 1;
        if previous_carry_flag {
            read_data |= 0b0000_0001;
        }
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);

        // AND
        self.reg_a &= read_data;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// RRA operations; ROR and then ADC
    pub fn op_rra(&mut self, bus: &mut Bus, data_ref: u16) {
        // ROR
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        let mut read_data = self.read_8bit(bus, data_ref);
        self.write_8bit(bus, data_ref, read_data);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b0000_0001 == 0b0000_0001);
        read_data >>= 1;
        if previous_carry_flag {
            read_data |= 0b1000_0000;
        }
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);

        // ADC
        self.reg_a = if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let temp_val = self.reg_a.wrapping_add(read_data).wrapping_add(1);
            let over_fl_st = (self.reg_a ^ temp_val) & (read_data ^ temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val <= self.reg_a);
            temp_val
        } else {
            let temp_val = self.reg_a.wrapping_add(read_data);
            let over_fl_st = (self.reg_a ^ temp_val) & (read_data ^ temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val < self.reg_a);
            temp_val
        };

        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// SLO / ASO operations; ASL then ORA
    pub fn op_slo(&mut self, bus: &mut Bus, data_ref: u16) {
        // ASL
        let mut read_data = self.read_8bit(bus, data_ref);
        self.write_8bit(bus, data_ref, read_data);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b1000_0000 == 0b1000_0000);
        read_data <<= 1;
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);

        // ORA
        self.reg_a |= read_data;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// SRE / LSE operations; LSR then EOR
    pub fn op_sre(&mut self, bus: &mut Bus, data_ref: u16) {
        // LSR
        let mut read_data = self.read_8bit(bus, data_ref);
        self.write_8bit(bus, data_ref, read_data);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b0000_0001 == 0b0000_0001);
        read_data >>= 1;
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);

        // EOR
        self.reg_a ^= read_data;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }
}
