use crate::cpu::Cpu;
use crate::cpu::CARRY_FLAG;
use crate::cpu::instructions::shared_ops::set_flag;

impl Cpu {
    /// DCP / DCM operations; Subtract 1 from memory (without borrow) then compare
    pub fn op_dcp(&mut self, data_ref: &mut u8) {
        *data_ref -= 1;
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= *data_ref);
    }

    /// ISC / ISB / INS operations; Add 1 from memory (without borrow) then compare
    pub fn op_isc(&mut self, _data_ref: &mut u8) {
    }
}
