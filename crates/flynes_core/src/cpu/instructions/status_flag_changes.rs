use crate::cpu::Cpu;
use crate::cpu::{CARRY_FLAG, DECIMAL_FLAG, INTERRUPT_FLAG, OVERFLOW_FLAG};
use crate::cpu::instructions::shared_ops::set_flag;

impl Cpu {
    /// Clear carry flag
    pub fn op_clc(&mut self) {
        set_flag(&mut self.cpu_status, CARRY_FLAG, false)
    }

    /// Clear decimal flag
    pub fn op_cld(&mut self) {
        set_flag(&mut self.cpu_status, DECIMAL_FLAG, false)
    }

    /// Clear interrupt flag
    pub fn op_cli(&mut self) {
        set_flag(&mut self.cpu_status, INTERRUPT_FLAG, false)
    }

    /// Clear overflow flag
    pub fn op_clv(&mut self) {
        set_flag(&mut self.cpu_status, OVERFLOW_FLAG, false)
    }

    /// Set carry flag
    pub fn op_sec(&mut self) {
        set_flag(&mut self.cpu_status, CARRY_FLAG, true)
    }

    /// Set decimal flag
    pub fn op_sed(&mut self) {
        set_flag(&mut self.cpu_status, DECIMAL_FLAG, true)
    }

    /// Set interrupt flag
    pub fn op_sei(&mut self) {
        set_flag(&mut self.cpu_status, INTERRUPT_FLAG, true)
    }
}

#[test]
fn test_status_flag_changes() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::instructions::shared_ops::is_flag_set;

    const CARRY_FLAG_MASK_INV: u8 = !(0b0000_0001 << CARRY_FLAG);
    const DECIMAL_FLAG_MASK_INV: u8 = !(0b0000_0001 << DECIMAL_FLAG);
    const INTERRUPT_MASK_INV: u8 = !(0b0000_0001 << INTERRUPT_FLAG);
    const OVERFLOW_MASK_INV: u8 = !(0b0000_0001 << OVERFLOW_FLAG);

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    for _ in 0..1000 {
        let random_status = rng.random::<u8>();
        cpu.cpu_status = random_status;

        cpu.op_clc();
        assert!(!is_flag_set(&cpu.cpu_status, CARRY_FLAG));
        assert_eq!(random_status & CARRY_FLAG_MASK_INV, cpu.cpu_status & CARRY_FLAG_MASK_INV);
        cpu.op_sec();
        assert!(is_flag_set(&cpu.cpu_status, CARRY_FLAG));
        assert_eq!(random_status & CARRY_FLAG_MASK_INV, cpu.cpu_status & CARRY_FLAG_MASK_INV);

        cpu.cpu_status = random_status;
        cpu.op_cld();
        assert!(!is_flag_set(&cpu.cpu_status, DECIMAL_FLAG));
        assert_eq!(random_status & DECIMAL_FLAG_MASK_INV, cpu.cpu_status & DECIMAL_FLAG_MASK_INV);
        cpu.op_sed();
        assert!(is_flag_set(&cpu.cpu_status, DECIMAL_FLAG));
        assert_eq!(random_status & DECIMAL_FLAG_MASK_INV, cpu.cpu_status & DECIMAL_FLAG_MASK_INV);

        cpu.cpu_status = random_status;
        cpu.op_cli();
        assert!(!is_flag_set(&cpu.cpu_status, INTERRUPT_FLAG));
        assert_eq!(random_status & INTERRUPT_MASK_INV, cpu.cpu_status & INTERRUPT_MASK_INV);
        cpu.op_sei();
        assert!(is_flag_set(&cpu.cpu_status, INTERRUPT_FLAG));
        assert_eq!(random_status & INTERRUPT_MASK_INV, cpu.cpu_status & INTERRUPT_MASK_INV);

        cpu.cpu_status = random_status;
        cpu.op_clv();
        assert!(!is_flag_set(&cpu.cpu_status, OVERFLOW_FLAG));
        assert_eq!(random_status & OVERFLOW_MASK_INV, cpu.cpu_status & OVERFLOW_MASK_INV);
    }
}
