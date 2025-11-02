use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::CARRY_FLAG;
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, set_flag, is_flag_set};

impl Cpu {
    /// Performs arithmetic shift left on memory
    pub fn op_asl(&mut self, bus: &mut Bus, data_ref: u16) {
        let mut read_data = self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b1000_0000 == 0b1000_0000);
        read_data <<= 1;
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data)
    }

    /// Performs arithmetic shift left on accumulator
    pub fn op_asl_acc(&mut self) {
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a & 0b1000_0000 == 0b1000_0000);
        self.reg_a <<= 1;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }

    /// Logical shift right for memory
    pub fn op_lsr(&mut self, bus: &mut Bus, data_ref: u16) {
        let mut read_data = self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b0000_0001 == 0b0000_0001);
        read_data >>= 1;
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data)
    }

    /// Logical shift right for accumulator
    pub fn op_lsr_acc(&mut self) {
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a & 0b0000_0001 == 0b0000_0001);
        self.reg_a >>= 1;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }

    /// Rotate left for memory
    pub fn op_rol(&mut self, bus: &mut Bus, data_ref: u16) {
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        let mut read_data = self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b1000_0000 == 0b1000_0000);
        read_data <<= 1;
        if previous_carry_flag {
            read_data |= 0b0000_0001;
        }
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);
    }

    /// Rotate left for accumulator
    pub fn op_rol_acc(&mut self) {
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a & 0b1000_0000 == 0b1000_0000);
        self.reg_a <<= 1;
        if previous_carry_flag {
            self.reg_a |= 0b0000_0001;
        }
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Rotate right for memory
    pub fn op_ror(&mut self, bus: &mut Bus, data_ref: u16) {
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        let mut read_data = self.read_8bit(bus, data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, read_data & 0b0000_0001 == 0b0000_0001);
        read_data >>= 1;
        if previous_carry_flag {
            read_data |= 0b1000_0000;
        }
        self.write_8bit(bus, data_ref, read_data);
        update_zero_and_neg_flags(&mut self.cpu_status, read_data);
    }

    /// Rotate right for accumulator
    pub fn op_ror_acc(&mut self) {
        let previous_carry_flag = is_flag_set(&self.cpu_status, CARRY_FLAG);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a & 0b0000_0001 == 0b0000_0001);
        self.reg_a >>= 1;
        if previous_carry_flag {
            self.reg_a |= 0b1000_0000;
        }
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }
}

#[test]
fn test_shifts() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};
    use crate::cpu::instructions::shared_ops::is_flag_set;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };
    let mut bus = Bus::default();

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);

        // ASL part
        cpu.op_asl(&mut bus, 0x0000);
        let mem_value = cpu.read_8bit(&mut bus, 0x0000u16);
        assert_eq!(mem_value, random_v << 1);
        assert!(mem_value.is_multiple_of(2));
        assert_eq!(is_flag_set(&cpu.cpu_status, CARRY_FLAG), random_v & 0b1000_0000 == 0b1000_0000);
        test_zero_and_neg(cpu.cpu_status, mem_value);
        let (mut old_cpu_status, mut old_mem_value) = (cpu.cpu_status, mem_value);

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);
        cpu.op_asl_acc();
        assert_eq!(cpu.reg_a, old_mem_value);
        assert_eq!(cpu.cpu_status, old_cpu_status);

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);

        // LSR part
        cpu.op_lsr(&mut bus, 0x0000);
        let mem_value = cpu.read_8bit(&mut bus, 0x0000u16);
        assert_eq!(mem_value, random_v >> 1);
        assert!(mem_value < 0b1000_0000);
        assert_eq!(is_flag_set(&cpu.cpu_status, CARRY_FLAG), random_v % 2 == 1);
        test_zero_and_neg(cpu.cpu_status, mem_value);
        (old_cpu_status, old_mem_value) = (cpu.cpu_status, mem_value);

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);
        cpu.op_lsr_acc();
        assert_eq!(cpu.reg_a, old_mem_value);
        assert_eq!(cpu.cpu_status, old_cpu_status);

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);

        let mut carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);
        let first_carry_flag = carry_flag_st;

        cpu.write_8bit(&mut bus, 0x0000u16, random_v);
        // ROL and ROR
        for _ in 0..random_st%16 {
            cpu.op_rol(&mut bus, 0x0000);
            let mem_value = cpu.read_8bit(&mut bus, 0x0000u16);
            assert_eq!(carry_flag_st, mem_value % 2 == 1);
            test_zero_and_neg(cpu.cpu_status, mem_value);
            carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);
        }

        for _ in 0..random_st%16 {
            cpu.op_ror(&mut bus, 0x0000);
            let mem_value = cpu.read_8bit(&mut bus, 0x0000u16);
            assert_eq!(carry_flag_st, mem_value & 0b1000_0000 == 0b1000_0000);
            test_zero_and_neg(cpu.cpu_status, mem_value);
            carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);
        }

        assert_eq!(first_carry_flag, is_flag_set(&cpu.cpu_status, CARRY_FLAG));
        assert_eq!(cpu.read_8bit(&mut bus, 0x0000u16), random_v);

        clear_cpu_and_mem(&mut cpu, &mut bus, random_st, random_v);
        let mut carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);

        // ROL acc and ROR acc
        for _ in 0..random_st%42 {
            cpu.op_rol_acc();
            assert_eq!(carry_flag_st, cpu.reg_a % 2 == 1);
            test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
            carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);
        }

        for _ in 0..random_st%42 {
            cpu.op_ror_acc();
            assert_eq!(carry_flag_st, cpu.reg_a & 0b1000_0000 == 0b1000_0000);
            test_zero_and_neg(cpu.cpu_status, cpu.reg_a);
            carry_flag_st = is_flag_set(&cpu.cpu_status, CARRY_FLAG);
        }

        assert_eq!(first_carry_flag, is_flag_set(&cpu.cpu_status, CARRY_FLAG));
        assert_eq!(cpu.reg_a, random_v);
    }

    fn clear_cpu_and_mem(cpu: &mut Cpu, bus: &mut Bus, random_st: u8, random_v: u8) {
        cpu.cpu_status = random_st;
        cpu.reg_a = random_v;
        cpu.write_8bit(bus, 0x0000u16, random_v);
    }

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
