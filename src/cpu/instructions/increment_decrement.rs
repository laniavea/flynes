use crate::cpu::Cpu;
use crate::cpu::instructions::shared_ops::update_zero_and_neg_flags;

impl Cpu {
    /// Increments memory
    /// Possible operation HEX: 0xE6, 0xF6, 0xEE, 0xFE
    pub fn op_inc(&mut self, data_ref: &mut u8) {
        *data_ref = data_ref.wrapping_add(1);
        update_zero_and_neg_flags(&mut self.cpu_status, *data_ref);
    }

    /// Increments register X
    /// Possible operation HEX: 0xE8
    pub fn op_inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x);
    }

    /// Increments register Y
    /// Possible operation HEX: 0xC8
    pub fn op_iny(&mut self) {
        self.reg_y = self.reg_y.wrapping_add(1);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_y);
    }

    /// Decrements memory
    /// Possible operation HEX: 0xC6, 0xD6, 0xCE, 0xDE
    pub fn op_dec(&mut self, data_ref: &mut u8) {
        *data_ref = data_ref.wrapping_sub(1);
        update_zero_and_neg_flags(&mut self.cpu_status, *data_ref);
    }

    /// Decrements register X
    /// Possible operation HEX: 0xCA
    pub fn op_dex(&mut self) {
        self.reg_x = self.reg_x.wrapping_sub(1);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_x);
    }

    /// Decrements register Y
    /// Possible operation HEX: 0x88
    pub fn op_dey(&mut self) {
        self.reg_y = self.reg_y.wrapping_sub(1);
        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_y);
    }
}

#[test]
fn test_incerement_decrement() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};
    use crate::cpu::instructions::shared_ops::is_flag_set;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        reg_x: 0,
        reg_y: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    let mut mem: [u8; 256] = [0u8; 256];

    for _ in 0..1000 {
        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        cpu.cpu_status = random_st;
        let random_ids: Vec<u8> = (0..random_v).map(|_| rng.random::<u8>()).collect();

        for now_id in &random_ids {
            cpu.op_inc(&mut mem[*now_id as usize]);
        }
        assert_eq!(mem.iter().sum::<u8>(), random_v);

        for now_id in &random_ids {
            cpu.op_dec(&mut mem[*now_id as usize]);
        }
        assert_eq!(mem.iter().sum::<u8>(), 0);

        cpu.reg_x = random_st;
        cpu.reg_y = random_st;

        for i in 0..random_v {
            cpu.op_inx();
            test_zero_and_neg(cpu.cpu_status, random_st.wrapping_add(i).wrapping_add(1));
            cpu.op_iny();
            test_zero_and_neg(cpu.cpu_status, random_st.wrapping_add(i).wrapping_add(1));
        }
        assert_eq!(cpu.reg_x, cpu.reg_y);
        assert_eq!(cpu.reg_x, random_st.wrapping_add(random_v));

        let new_start = random_st.wrapping_add(random_v);
        for i in 0..random_v {
            cpu.op_dex();
            test_zero_and_neg(cpu.cpu_status, new_start.wrapping_sub(i).wrapping_sub(1));
            cpu.op_dey();
            test_zero_and_neg(cpu.cpu_status, new_start.wrapping_sub(i).wrapping_sub(1));
        }
        assert_eq!(cpu.reg_x, cpu.reg_y);
        assert_eq!(cpu.reg_x, random_st);
    }


    let mut now_counter: u8 = 0;
    let number_of_iters: usize = 256+10;

    for i in 0..number_of_iters {
        cpu.op_inc(&mut now_counter);
        test_zero_and_neg(cpu.cpu_status, ((i+1) % 256) as u8);
        assert_eq!(now_counter, ((i+1) % 256) as u8);
    }
    assert_eq!(now_counter, 10);

    for i in 0..number_of_iters {
        cpu.op_dec(&mut now_counter);
        test_zero_and_neg(cpu.cpu_status, ((number_of_iters - i - 1) % 256) as u8);
        assert_eq!(now_counter, ((number_of_iters - i - 1) % 256) as u8);
    }
    assert_eq!(now_counter, 0);

    fn test_zero_and_neg(cpu_status: u8, target_value: u8) {
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), target_value == 0);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), target_value >= 0b1000_0000);
    }
}
