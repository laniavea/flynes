use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

impl Cpu {
    /// Sets memory value to register A, updates Zero and Neg flags
    pub fn op_lda(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a = self.read_8bit(bus, data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }

    /// Sets memory value to register X, updates Zero and Neg flags
    pub fn op_ldx(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_x = self.read_8bit(bus, data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x)
    }

    /// Sets memory value to register Y, updates Zero and Neg flags
    pub fn op_ldy(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_y = self.read_8bit(bus, data_ref);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_y)
    }

    /// Sets value of register A to memory
    pub fn op_sta(&mut self, bus: &mut Bus, data_ref: u16) {
        self.write_8bit(bus, data_ref, self.reg_a);
    }

    /// Sets value of register X to memory
    pub fn op_stx(&mut self, bus: &mut Bus, data_ref: u16) {
        self.write_8bit(bus, data_ref, self.reg_x);
    }

    /// Sets value of register Y to memory
    pub fn op_sty(&mut self, bus: &mut Bus, data_ref: u16) {
        self.write_8bit(bus, data_ref, self.reg_y);
    }
}

#[test]
fn test_load_store_ops() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::instructions::shared_ops::is_flag_set;
    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};
    use crate::memory::RAM;

    const OFFSET: usize = 256;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        reg_x: 0,
        reg_y: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    let mut bus = Bus::default();
    *bus.memory_mut().ram_mut() = [6; RAM.size];

    for now_value in (0..OFFSET).rev() {
        let now_v: u8 = now_value.try_into().unwrap();
        cpu.write_8bit(&mut bus, RAM.end, now_v);
        let random_v = rng.random::<u8>();

        cpu.reg_a = now_v.wrapping_add(10);
        cpu.reg_x = now_v.wrapping_add(10);
        cpu.reg_y = now_v.wrapping_add(10);
        cpu.cpu_status = random_v;

        cpu.op_lda(&mut bus, RAM.end as u16);
        let new_status = cpu.cpu_status;

        cpu.cpu_status = random_v;
        cpu.op_ldx(&mut bus, RAM.end as u16);
        assert_eq!(new_status, cpu.cpu_status);

        cpu.cpu_status = random_v;
        cpu.op_ldy(&mut bus, RAM.end as u16);
        assert_eq!(new_status, cpu.cpu_status);

        assert_eq!(is_flag_set(&cpu.cpu_status, ZERO_FLAG), now_value == 0);
        assert_eq!(is_flag_set(&cpu.cpu_status, NEGATIVE_FLAG), now_value >= 0b1000_0000);
        assert_eq!([cpu.reg_a, cpu.reg_x, cpu.reg_y], [now_value as u8; 3]);

        cpu.op_sta(&mut bus, now_value as u16);
        cpu.op_stx(&mut bus, (now_value + OFFSET) as u16);
        cpu.op_sty(&mut bus, (now_value + (OFFSET * 2)) as u16);
    }

    for (now_id, now_v) in bus.memory_mut().ram().iter().enumerate().take(OFFSET * 3) {
        assert_eq!(*now_v, (now_id % OFFSET) as u8);
    }
}
