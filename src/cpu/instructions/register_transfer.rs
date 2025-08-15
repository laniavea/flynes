use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

impl Cpu {
    /// Transfers register A to register X
    pub fn op_tax(&mut self) {
        self.reg_x = self.reg_a;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x)
    }

    /// Transfers register A to register Y
    pub fn op_tay(&mut self) {
        self.reg_y = self.reg_a;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_y)
    }

    /// Transfers register X to register A
    pub fn op_txa(&mut self) {
        self.reg_a = self.reg_x;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }

    /// Transfers register Y to register A
    pub fn op_tya(&mut self) {
        self.reg_a = self.reg_y;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }
}

#[test]
fn test_register_transfer() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::instructions::shared_ops::is_flag_set;
    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        reg_x: 0,
        reg_y: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();
        let first_random_offset = rng.random::<u8>();
        let second_random_offset = rng.random::<u8>();

        cpu.reg_a = random_v.wrapping_add(first_random_offset);
        cpu.reg_y = random_v.wrapping_add(second_random_offset);
        cpu.reg_x = random_v;

        cpu.cpu_status = random_st;
        cpu.op_txa();
        test_zero_and_neg(cpu.cpu_status, random_v);

        cpu.cpu_status = random_st;
        cpu.op_tay();
        test_zero_and_neg(cpu.cpu_status, random_v);

        assert_eq!([cpu.reg_a, cpu.reg_x, cpu.reg_y], [random_v; 3]);

        cpu.reg_a = random_v.wrapping_add(second_random_offset);
        cpu.reg_x = random_v.wrapping_add(first_random_offset);

        cpu.cpu_status = random_st;
        cpu.op_tya();
        test_zero_and_neg(cpu.cpu_status, random_v);

        cpu.cpu_status = random_st;
        cpu.op_tax();
        test_zero_and_neg(cpu.cpu_status, random_v);

        assert_eq!([cpu.reg_a, cpu.reg_x, cpu.reg_y], [random_v; 3]);
    }

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
