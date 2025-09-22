use crate::cpu::Cpu;
use crate::cpu::CpuState;

impl Cpu {
    pub fn op_shx(&mut self, data_address: u16) {
        self.reg_x &= ((data_address >> 8) as u8).wrapping_add(1);
    }

    pub fn op_shy(&mut self, data_address: u16) {
        self.reg_y &= ((data_address >> 8) as u8).wrapping_add(1);
    }

    pub fn op_stp(&mut self) {
        self.state = CpuState::Stopped;
    }
}
