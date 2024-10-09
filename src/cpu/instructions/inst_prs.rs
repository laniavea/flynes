use crate::cpu::Cpu;

impl Cpu {
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
