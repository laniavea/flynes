use crate::cpu::Cpu;
use crate::cpu::CARRY_FLAG;
use crate::cpu::instructions::shared_ops::{set_flag, update_zero_and_neg_flags};

impl Cpu {
    /// DCP / DCM operations; Subtract 1 from memory (without borrow) then compare
    pub fn op_dcp(&mut self, data_ref: &mut u8) {
        *data_ref = data_ref.wrapping_sub(1);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= *data_ref);
        let temp_res = self.reg_a.wrapping_sub(*data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, temp_res);
    }

    /// ISC / ISB / INS operations; Add 1 from memory (without borrow) then SBC
    pub fn op_isc(&mut self, data_ref: &mut u8) {
        *data_ref = data_ref.wrapping_add(1);
        self.op_sbc(data_ref);
    }

    /// RLA operations; ROL and then AND;
    pub fn op_rla(&mut self, data_ref: &mut u8) {
        self.op_rol(data_ref);
        self.reg_a &= *data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// RRA operations; ROR and then ADC
    pub fn op_rra(&mut self, data_ref: &mut u8) {
        self.op_ror(data_ref);
        self.op_adc(data_ref);
    }

    /// SLO / ASO operations; ASL then ORA
    pub fn op_slo(&mut self, data_ref: &mut u8) {
        self.op_asl(data_ref);
        self.op_ora(data_ref);
    }

    /// SRE / LSE operations; LSR then EOR
    pub fn op_sre(&mut self, data_ref: &mut u8) {
        self.op_lsr(data_ref);
        self.op_eor(data_ref);
    }
}
