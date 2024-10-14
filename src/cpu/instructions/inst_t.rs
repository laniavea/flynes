use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::*;

// All obelisk 6502 instructions which starts with T
// All of them just copy values from one register to another with Zero an Neg flags update
// Instructions here: TAX, TAY, TSX, TXA, TXS, TYA
// More info: https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
impl Cpu {
    pub fn op_tax(&mut self) {
        self.reg_x = self.reg_a;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_tay(&mut self) {
        self.reg_y = self.reg_a;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_y);
    }

    pub fn op_tsx(&mut self) {
        self.reg_x = self.stack_pointer;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_x);
    }

    pub fn op_txa(&mut self) {
        self.reg_a = self.reg_x;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }

    pub fn op_txs(&mut self) {
        self.stack_pointer = self.reg_x;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.stack_pointer);
    }

    pub fn op_tya(&mut self) {
        self.reg_a = self.reg_y;
        self.cpu_status = update_zero_and_neg_flags(self.cpu_status, self.reg_a);
    }
}

#[test]
fn test_t_operations() {
    let mut cpu = Cpu::new();

    cpu.reg_x = 0;
    cpu.reg_y = 0;
    cpu.reg_a = 255;

    cpu.op_tax();
    cpu.op_tay();
    assert_eq!((cpu.reg_x, cpu.reg_y, cpu.reg_a), (255, 255, 255));

    cpu.stack_pointer = 0;
    cpu.op_tsx();
    cpu.op_txa();
    assert_eq!(cpu.reg_a, cpu.stack_pointer);

    cpu.reg_x = 127;
    cpu.reg_y = 127;
    cpu.op_txs();
    cpu.op_tya();
    assert_eq!(cpu.stack_pointer, cpu.reg_a);
}
