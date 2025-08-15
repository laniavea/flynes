use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

impl Cpu {
    /// Sets memory value to register A, updates Zero and Neg flags
    pub fn op_lda(&mut self, data_ref: &u8) {
        self.reg_a = *data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a)
    }

    /// Sets memory value to register X, updates Zero and Neg flags
    pub fn op_ldx(&mut self, data_ref: &u8) {
        self.reg_x = *data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x)
    }

    /// Sets memory value to register Y, updates Zero and Neg flags
    pub fn op_ldy(&mut self, data_ref: &u8) {
        self.reg_y = *data_ref;
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_y)
    }

    /// Sets value of register A to memory
    pub fn op_sta(&mut self, data_ref: &mut u8) {
        *data_ref = self.reg_a;
    }

    /// Sets value of register X to memory
    pub fn op_stx(&mut self, data_ref: &mut u8) {
        *data_ref = self.reg_x;
    }

    /// Sets value of register Y to memory
    pub fn op_sty(&mut self, data_ref: &mut u8) {
        *data_ref = self.reg_y;
    }
}

#[test]
fn test_load_store_ops() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::instructions::shared_ops::is_flag_set;
    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};

    const OFFSET: usize = 256;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        reg_x: 0,
        reg_y: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    let mut mem: [u8; OFFSET * 3] = [6; OFFSET * 3];

    for now_value in (0..OFFSET).rev() {
        let now_v: &u8 = &mut now_value.try_into().unwrap();
        let random_v = rng.random::<u8>();

        cpu.reg_a = now_v.wrapping_add(10);
        cpu.reg_x = now_v.wrapping_add(10);
        cpu.reg_y = now_v.wrapping_add(10);
        cpu.cpu_status = random_v;

        cpu.op_lda(now_v);
        let new_status = cpu.cpu_status;

        cpu.cpu_status = random_v;
        cpu.op_ldx(now_v);
        assert_eq!(new_status, cpu.cpu_status);

        cpu.cpu_status = random_v;
        cpu.op_ldy(now_v);
        assert_eq!(new_status, cpu.cpu_status);

        assert_eq!(is_flag_set(&cpu.cpu_status, ZERO_FLAG), now_value == 0);
        assert_eq!(is_flag_set(&cpu.cpu_status, NEGATIVE_FLAG), now_value >= 0b1000_0000);
        assert_eq!([cpu.reg_a, cpu.reg_x, cpu.reg_y], [now_value as u8; 3]);

        cpu.op_sta(mem.get_mut(now_value).unwrap());
        cpu.op_stx(mem.get_mut(now_value + OFFSET).unwrap());
        cpu.op_sty(mem.get_mut(now_value + (OFFSET * 2)).unwrap());
    }

    for (now_id, now_v) in mem.iter().enumerate() {
        assert_eq!(*now_v, (now_id % OFFSET) as u8);
    }
}
