use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::CARRY_FLAG;
use crate::cpu::instructions::shared_ops::{set_flag, update_zero_and_neg_flags};

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
        self.op_sbc(bus, data_ref);
    }

    /// RLA operations; ROL and then AND;
    pub fn op_rla(&mut self, bus: &mut Bus, data_ref: u16) {
        self.op_rol(bus, data_ref);
        self.reg_a &= self.read_8bit(bus, data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// RRA operations; ROR and then ADC
    pub fn op_rra(&mut self, bus: &mut Bus, data_ref: u16) {
        self.op_ror(bus, data_ref);
        self.op_adc(bus, data_ref);
    }

    /// SLO / ASO operations; ASL then ORA
    pub fn op_slo(&mut self, bus: &mut Bus, data_ref: u16) {
        self.op_asl(bus, data_ref);
        self.op_ora(bus, data_ref);
    }

    /// SRE / LSE operations; LSR then EOR
    pub fn op_sre(&mut self, bus: &mut Bus, data_ref: u16) {
        self.op_lsr(bus, data_ref);
        self.op_eor(bus, data_ref);
    }
}
