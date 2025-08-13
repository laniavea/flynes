use crate::cpu::Cpu;
use crate::cpu::{CARRY_FLAG, OVERFLOW_FLAG};
use crate::cpu::instructions::shared_ops::{update_zero_and_neg_flags, is_flag_set, set_flag};

impl Cpu {
    /// Writes add with carry to reg A by formula A(reg) + M(emory) + C(arry)
    /// Possible operation HEX: 0x69, 0x65, 0x75, 0x6D, 0x7D, 0x79, 0x61, 0x71
    pub fn op_adc(&mut self, data_ref: &u8) {
        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        // Set carry if result < reg_a -> reg_a + data > 255
        // A + M + C

        self.reg_a = if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let temp_val = self.reg_a.wrapping_add(*data_ref).wrapping_add(1);
            let over_fl_st = (self.reg_a^temp_val) & (*data_ref^temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val <= self.reg_a);
            temp_val
        } else {
            let temp_val = self.reg_a.wrapping_add(*data_ref);
            let over_fl_st = (self.reg_a^temp_val) & (*data_ref^temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val < self.reg_a);
            temp_val
        };

        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Writes substract with carry to reg A by formula A(reg) - M(emory) - (C(arry) - 1)
    /// Possible operation HEX: 0xE9, 0xE5, 0xF5, 0xED, 0xFD, 0xF9, 0xE1, 0xF1
    pub fn op_sbc(&mut self, data_ref: &u8) {
        // Set overflow if first bits were same, but result's first bit isn't same (11 0 -> Overflow)
        // Set carry if result < reg_a -> reg_a + data > 255
        // A - M - (C - 1)

        self.reg_a = if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let temp_val = self.reg_a.wrapping_sub(*data_ref);
            let over_fl_st = (self.reg_a^temp_val) & ((0b1111_1111 - *data_ref)^temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val <= self.reg_a);
            temp_val
        } else {
            let temp_val = self.reg_a.wrapping_sub(*data_ref).wrapping_sub(1);
            let over_fl_st = (self.reg_a^temp_val) & ((0b1111_1111 - *data_ref)^temp_val) & 0b1000_0000 != 0;

            set_flag(&mut self.cpu_status, OVERFLOW_FLAG, over_fl_st);
            set_flag(&mut self.cpu_status, CARRY_FLAG, temp_val < self.reg_a);
            temp_val
        };

        update_zero_and_neg_flags(&mut self.cpu_status, self.reg_a);
    }

    /// Compares memory with register A, changes cpu status
    /// Possible operation HEX: 0xC9, 0xC5, 0xD5, 0xCD, 0xDD, 0xD9, 0xC1, 0xD1
    pub fn op_cmp(&mut self, data_ref: &u8) {
        let temp_res = self.reg_a.wrapping_sub(*data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= temp_res);
        update_zero_and_neg_flags(&mut self.cpu_status, temp_res);
    }

    /// Compares memory with register X, changes cpu status
    /// Possible operation HEX: 0xE0, 0xE4, 0xEC
    pub fn op_cpx(&mut self, data_ref: &u8) {
        let temp_res = self.reg_x.wrapping_sub(*data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= temp_res);
        update_zero_and_neg_flags(&mut self.cpu_status, temp_res);
    }

    /// Compares memory with register Y, changes cpu status
    /// Possible operation HEX: 0xC0, 0xC4, 0xCC
    pub fn op_cpy(&mut self, data_ref: &u8) {
        let temp_res = self.reg_y.wrapping_sub(*data_ref);
        set_flag(&mut self.cpu_status, CARRY_FLAG, self.reg_a >= temp_res);
        update_zero_and_neg_flags(&mut self.cpu_status, temp_res);
    }
}

#[test]
fn test_arithmetic() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::instructions::shared_ops::is_flag_set;
    use crate::cpu::{ZERO_FLAG, NEGATIVE_FLAG};

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        reg_a: 0,
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    // ASM code for tests below
    // ----------------------
    // LDA #$90
    // ADC #$90
    // ADC #$20
    // ADC #$50
    // ADC #$6F
    // ADC #$FF
    // ADC #$7F
    // ADC #$00
    // ADC #$80
    // LDA #$7F
    // CLC
    // ADC #$01
    // ADC #$FF

    cpu.reg_a = 0x90;
    cpu.op_adc(&0x90);
    check_arith(cpu.reg_a, 0x20, cpu.cpu_status, [false, true, false, true]);
    cpu.op_adc(&0x20);
    check_arith(cpu.reg_a, 0x41, cpu.cpu_status, [false, false, false, false]);
    cpu.op_adc(&0x50);
    check_arith(cpu.reg_a, 0x91, cpu.cpu_status, [true, true, false, false]);
    cpu.op_adc(&0x6F);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    cpu.op_adc(&0xFF);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    cpu.op_adc(&0x7F);
    check_arith(cpu.reg_a, 0x80, cpu.cpu_status, [true, true, false, false]);
    cpu.op_adc(&0x00);
    check_arith(cpu.reg_a, 0x80, cpu.cpu_status, [true, false, false, false]);
    cpu.op_adc(&0x80);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, true, true, true]);
    cpu.reg_a = 0x7F;
    set_flag(&mut cpu.cpu_status, CARRY_FLAG, false);
    cpu.op_adc(&0x01);
    check_arith(cpu.reg_a, 0x80, cpu.cpu_status, [true, true, false, false]);
    cpu.op_adc(&0xFF);
    check_arith(cpu.reg_a, 0x7F, cpu.cpu_status, [false, true, false, true]);


    // ASM code for tests below
    // -----------------------
    // LDA #$0
    // SBC #$64
    // SBC #$9B
    // SBC #$01
    // SBC #$FC
    // SBC #$02
    // SBC #$80
    // SBC #$80
    // SBC #$FF
    // SBC #$FD
    // SBC #$00
    // SBC #$FF
    // SBC #$00
    // SBC #$00
    // CLC
    // SBC #$01

    cpu.cpu_status = 0b0000_0000;
    cpu.reg_a = 0x0;
    cpu.op_sbc(&0x64);
    check_arith(cpu.reg_a, 0x9B, cpu.cpu_status, [true, false, false, false]);
    cpu.op_sbc(&0x9B);
    check_arith(cpu.reg_a, 0xFF, cpu.cpu_status, [true, false, false, false]);
    cpu.op_sbc(&0x01);
    check_arith(cpu.reg_a, 0xFD, cpu.cpu_status, [true, false, false, true]);
    cpu.op_sbc(&0xFC);
    check_arith(cpu.reg_a, 0x01, cpu.cpu_status, [false, false, false, true]);
    cpu.op_sbc(&0x02);
    check_arith(cpu.reg_a, 0xFF, cpu.cpu_status, [true, false, false, false]);
    cpu.op_sbc(&0x80);
    check_arith(cpu.reg_a, 0x7E, cpu.cpu_status, [false, false, false, true]);
    cpu.op_sbc(&0x80);
    check_arith(cpu.reg_a, 0xFE, cpu.cpu_status, [true, true, false, false]);
    cpu.op_sbc(&0xFF);
    check_arith(cpu.reg_a, 0xFE, cpu.cpu_status, [true, false, false, false]);
    cpu.op_sbc(&0xFD);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    cpu.op_sbc(&0x00);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    cpu.op_sbc(&0xFF);
    check_arith(cpu.reg_a, 0x01, cpu.cpu_status, [false, false, false, false]);
    cpu.op_sbc(&0x00);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    cpu.op_sbc(&0x00);
    check_arith(cpu.reg_a, 0x00, cpu.cpu_status, [false, false, true, true]);
    set_flag(&mut cpu.cpu_status, CARRY_FLAG, false);
    cpu.op_sbc(&0x01);
    check_arith(cpu.reg_a, 0xFE, cpu.cpu_status, [true, false, false, false]);

    cpu.reg_a = 0x80;
    set_flag(&mut cpu.cpu_status, CARRY_FLAG, true);
    cpu.op_sbc(&0x7F);

    assert_eq!(cpu.reg_a, 0x01);
    assert!(is_flag_set(&cpu.cpu_status, OVERFLOW_FLAG));

    let mut last_val = rng.random::<u8>();
    for _ in 0..1000 {
        cpu.reg_a = last_val;
        cpu.reg_x = last_val;
        cpu.reg_y = last_val;

        let random_v = rng.random::<u8>();
        let random_st = rng.random::<u8>();

        cpu.cpu_status = random_st;
        cpu.op_cmp(&random_v);
        let new_cpu_status = cpu.cpu_status;

        cpu.cpu_status = random_st;
        cpu.op_cpx(&random_v);
        assert_eq!(cpu.cpu_status, new_cpu_status);

        cpu.cpu_status = random_st;
        cpu.op_cpy(&random_v);
        assert_eq!(cpu.cpu_status, new_cpu_status);

        assert_eq!(is_flag_set(&cpu.cpu_status, NEGATIVE_FLAG), last_val.wrapping_sub(random_v) & 0b1000_0000 != 0);
        assert_eq!(is_flag_set(&cpu.cpu_status, CARRY_FLAG), last_val >= random_v);
        assert_eq!(is_flag_set(&cpu.cpu_status, ZERO_FLAG), last_val == random_v);

        last_val = random_v;
    }

    // flags are NEGATIVE -> OVERFLOW -> ZERO -> CARRY
    fn check_arith(res: u8, exp_res: u8, cpu_status: u8, flags: [bool; 4]) {
        assert_eq!(res, exp_res);
        assert_eq!(is_flag_set(&cpu_status, NEGATIVE_FLAG), flags[0]);
        assert_eq!(is_flag_set(&cpu_status, OVERFLOW_FLAG), flags[1]);
        assert_eq!(is_flag_set(&cpu_status, ZERO_FLAG), flags[2]);
        assert_eq!(is_flag_set(&cpu_status, CARRY_FLAG), flags[3]);
    }
}
