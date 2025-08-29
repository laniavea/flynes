use better_assertions::inst_assert;

use crate::cpu::Cpu;
use crate::cpu::{BREAK_FLAG, UNUSED_FLAG};
use crate::cpu::instructions::shared_ops::{set_flag, is_flag_set};
use crate::memory::Memory;

impl Cpu {
    /// Creates forced interrupt
    pub fn op_brk(&mut self, mem: &mut Memory) {
        set_flag(&mut self.cpu_status, BREAK_FLAG, true);
        inst_assert!(is_flag_set(&self.cpu_status, UNUSED_FLAG));
        mem.stack_push_16bit(self.program_counter, &mut self.stack_pointer);
        mem.stack_push_8bit(self.cpu_status, &mut self.stack_pointer);
        self.program_counter = mem.get_16bit_value(0xFFFE);
    }

    /// Do literally nothing outside normal change of PC
    pub fn op_nop(&mut self) {
    }

    /// Return from interrupt, pulls cpu status and pc from stack
    pub fn op_rti(&mut self, mem: &Memory) {
        self.cpu_status = mem.stack_pull_8bit(&mut self.stack_pointer);
        self.program_counter = mem.stack_pull_16bit(&mut self.stack_pointer);
    }
}

#[test]
fn test_system_functions() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu: Cpu = Cpu {
        stack_pointer: 0xFF,
        program_counter: 0u16,
        cpu_status: 0u8,
        ..Default::default()
    };

    let mut memory = Memory::default();

    for _ in 0..1000 {
        let old_random_pc = rng.random::<u16>();
        let random_cpu_status = rng.random::<u8>() | (0b0000_0001 << UNUSED_FLAG);
        let new_random_pc = rng.random::<u16>();

        *memory.get_mut_8bit_value(0xFFFEu16) = new_random_pc as u8;
        *memory.get_mut_8bit_value(0xFFFFu16) = (new_random_pc >> 8) as u8;

        cpu.cpu_status = random_cpu_status;
        cpu.program_counter = old_random_pc;
        cpu.stack_pointer = 0xFF;

        cpu.op_brk(&mut memory);

        assert_eq!(cpu.program_counter, new_random_pc);
        assert_eq!(cpu.stack_pointer, 0xFC);
        assert_eq!(cpu.cpu_status, random_cpu_status | (0b0000_0001 << BREAK_FLAG));

        cpu.op_rti(&memory);
        assert_eq!(cpu.cpu_status, random_cpu_status | (0b0000_0001 << BREAK_FLAG));
        assert_eq!(cpu.program_counter, old_random_pc);
    }
}
