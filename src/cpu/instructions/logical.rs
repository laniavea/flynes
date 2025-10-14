use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::{ZERO_FLAG, OVERFLOW_FLAG, NEGATIVE_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, set_flag, transfer_bit};

impl Cpu {
    /// Perfomrs logical AND between register A and data, result saved in reg A
    pub fn op_and(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a &= bus.read_8bit_cpu(data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Perfomrs logical XOR between register A and data, result saved in reg A
    pub fn op_eor(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a ^= bus.read_8bit_cpu(data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Performs logical OR between register A and data, result saved in reg A
    pub fn op_ora(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a |= bus.read_8bit_cpu(data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Performs logical AND between register A and data, affects only to cpu status
    pub fn op_bit(&mut self, bus: &mut Bus, data_ref: u16) {
        let read_data = bus.read_8bit_cpu(data_ref);
        set_flag(&mut self.cpu_status, ZERO_FLAG, (read_data & self.reg_a) == 0);
        transfer_bit(&mut self.cpu_status, &read_data, OVERFLOW_FLAG);
        transfer_bit(&mut self.cpu_status, &read_data, NEGATIVE_FLAG);
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
    let mut bus: Bus = Bus::default();

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        cpu.reg_a = random_v;
        bus.write_8bit_cpu(0x0000u16, 0b0000_0000);
        bus.write_8bit_cpu(0x0001u16, 0b1111_1111);
        bus.write_8bit_cpu(0x0002u16, !random_v);

        let other_random_v = rng.random::<u8>();
        bus.write_8bit_cpu(0x0003u16, other_random_v);

        cpu.op_ora(&mut bus, 0x0000);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        cpu.op_and(&mut bus, 0x0001);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        cpu.op_eor(&mut bus, 0x0000);
        test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
        assert_eq!(cpu.reg_a, random_v);

        cpu.cpu_status = random_st;
        cpu.op_ora(&mut bus, 0x0002);
        assert_eq!(cpu.reg_a, 0b1111_1111);
        test_zero_and_neg(cpu.cpu_status, 0b1111_1111);

        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_and(&mut bus, 0x0002);
        assert_eq!(cpu.reg_a, 0b0000_0000);
        test_zero_and_neg(cpu.cpu_status, 0b0000_0000);

        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_eor(&mut bus, 0x0002);
        assert_eq!(cpu.reg_a, 0b1111_1111);
        test_zero_and_neg(cpu.cpu_status, 0b1111_1111);

        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.op_bit(&mut bus, 0x0003);
        assert_eq!(is_flag_set(&cpu.cpu_status, ZERO_FLAG), other_random_v & random_v == 0);
        assert_eq!(is_flag_set(&cpu.cpu_status, NEGATIVE_FLAG), is_flag_set(&other_random_v, NEGATIVE_FLAG));
        assert_eq!(is_flag_set(&cpu.cpu_status, OVERFLOW_FLAG), is_flag_set(&other_random_v, OVERFLOW_FLAG));
    }

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
