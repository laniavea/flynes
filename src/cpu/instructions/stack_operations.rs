use better_assertions::inst_assert;

use crate::cpu::Cpu;
use crate::cpu::{UNUSED_FLAG, BREAK_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, is_flag_set};
use crate::bus::Bus;

const UNUSED_FLAG_BIT: u8 = 0b0000_0001 << UNUSED_FLAG;
const BREAK_FLAG_BIT: u8 = 0b0000_0001 << BREAK_FLAG;
const BREAK_FLAG_REVERSED_BIT: u8 = !BREAK_FLAG_BIT;

impl Cpu {
    /// Transfers stack pointer to register X
    pub fn op_tsx(&mut self) {
        self.reg_x = self.stack_pointer;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x);
    }

    /// Transfers register X to stack pointer
    pub fn op_txs(&mut self) {
        self.stack_pointer = self.reg_x;
    }

    /// Pushes register A to stack
    pub fn op_pha(&mut self, bus: &mut Bus) {
        bus.memory_mut().stack_push_8bit(self.reg_a, &mut self.stack_pointer)
    }

    /// Pushes cpu status to stack
    pub fn op_php(&mut self, bus: &mut Bus) {
        inst_assert!(is_flag_set(&self.cpu_status, UNUSED_FLAG));
        bus.memory_mut().stack_push_8bit(self.cpu_status | BREAK_FLAG_BIT, &mut self.stack_pointer);
    }

    /// Pulls actual stack value to register A
    pub fn op_pla(&mut self, bus: &Bus) {
        self.reg_a = bus.memory().stack_pull_8bit(&mut self.stack_pointer);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Pulls actual stack value to cpu status
    pub fn op_plp(&mut self, bus: &Bus) {
        self.cpu_status = bus.memory().stack_pull_8bit(&mut self.stack_pointer) | UNUSED_FLAG_BIT;
        self.cpu_status &= BREAK_FLAG_REVERSED_BIT;
    }
}

#[test]
fn test_stack_operations() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        reg_x: 0,
        cpu_status: 0b0000_0000,
        stack_pointer: 0,
        ..Default::default()
    };

    let mut bus: Bus = Bus::default();

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        cpu.cpu_status = random_st;
        cpu.stack_pointer = random_v;
        cpu.op_tsx();
        test_zero_and_neg(cpu.cpu_status, random_v);
        assert_eq!(cpu.reg_x, random_v);

        cpu.stack_pointer = random_v.wrapping_add(10);
        cpu.op_txs();
        assert_eq!(cpu.stack_pointer, random_v);

        cpu.reg_a = random_st;
        cpu.op_pha(&mut bus);
        assert_eq!(bus.memory().stack_as_slice()[cpu.stack_pointer.wrapping_add(1) as usize], random_st);

        cpu.reg_a = random_v;
        cpu.op_pla(&bus);
        assert_eq!(cpu.reg_a, random_st);
        test_zero_and_neg(cpu.cpu_status, random_st);

        cpu.cpu_status = random_v | UNUSED_FLAG_BIT;
        cpu.op_php(&mut bus);
        assert_eq!(
            bus.memory().stack_as_slice()[cpu.stack_pointer.wrapping_add(1) as usize],
            random_v | UNUSED_FLAG_BIT | BREAK_FLAG_BIT
        );

        cpu.cpu_status = cpu.cpu_status.wrapping_add(random_v);
        cpu.op_plp(&bus);
        assert_eq!(cpu.cpu_status, (random_v | UNUSED_FLAG_BIT) & BREAK_FLAG_REVERSED_BIT);
    }

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
