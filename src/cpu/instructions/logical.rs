use crate::cpu::Cpu;
use crate::cpu::{ZERO_FLAG, OVERFLOW_FLAG, NEGATIVE_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, set_flag, transfer_bit};

impl Cpu {
    /// Perfomrs logical AND between register A and data, result saved in reg A
    pub fn op_and(&mut self, data_ref: &u8) {
        self.reg_a &= data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Perfomrs logical XOR between register A and data, result saved in reg A
    pub fn op_eor(&mut self, data_ref: &u8) {
        self.reg_a ^= data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Performs logical OR between register A and data, result saved in reg A
    pub fn op_ora(&mut self, data_ref: &u8) {
        self.reg_a |= data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Performs logical AND between register A and data, affects only to cpu status
    pub fn op_bit(&mut self, data_ref: &u8) {
        set_flag(&mut self.cpu_status, ZERO_FLAG, (data_ref & self.reg_a) == 0);
        transfer_bit(&mut self.cpu_status, data_ref, OVERFLOW_FLAG);
        transfer_bit(&mut self.cpu_status, data_ref, NEGATIVE_FLAG);
    }
}

#[test]
fn test_logical_operations() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG};
    use crate::cpu::instructions::shared_ops::is_flag_set;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        cpu.reg_a = random_v;
        cpu.op_ora(&0b0000_0000);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        cpu.op_and(&0b1111_1111);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        cpu.op_eor(&0b0000_0000);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        assert_eq!(cpu.reg_a, random_v);

        cpu.cpu_status = random_st;
        cpu.op_ora(&(!random_v));
        assert_eq!(cpu.reg_a, 0b1111_1111);
        test_zero_and_neg(cpu.cpu_status, 0b1111_1111);

        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_and(&(!random_v));
        assert_eq!(cpu.reg_a, 0b0000_0000);
        test_zero_and_neg(cpu.cpu_status, 0b0000_0000);

        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_eor(&(!random_v));
        assert_eq!(cpu.reg_a, 0b1111_1111);
        test_zero_and_neg(cpu.cpu_status, 0b1111_1111);

        let other_random_v = rng.random::<u8>();
        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_bit(&other_random_v);
        assert_eq!(is_flag_set(&cpu.cpu_status, ZERO_FLAG), other_random_v & random_v == 0);
        assert_eq!(is_flag_set(&cpu.cpu_status, NEGATIVE_FLAG), is_flag_set(&other_random_v, NEGATIVE_FLAG));
        assert_eq!(is_flag_set(&cpu.cpu_status, OVERFLOW_FLAG), is_flag_set(&other_random_v, OVERFLOW_FLAG));
    }

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
