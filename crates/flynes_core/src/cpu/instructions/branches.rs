use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::instructions::shared_ops::is_flag_set;
use crate::cpu::{CARRY_FLAG, ZERO_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG};

impl Cpu {
    /// Branch if carry flag set
    /// Possible operation HEX: 0xB0
    pub fn op_bcs(&mut self, bus: &mut Bus, data_ref: u16) {
        if is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if carry flag clear
    /// Possible operation HEX: 0x90
    pub fn op_bcc(&mut self, bus: &mut Bus, data_ref: u16) {
        if !is_flag_set(&self.cpu_status, CARRY_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if zero flag set
    /// Possible operation HEX: 0xF0
    pub fn op_beq(&mut self, bus: &mut Bus, data_ref: u16) {
        if is_flag_set(&self.cpu_status, ZERO_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if zero flag clear
    /// Possible operation HEX: 0xD0
    pub fn op_bne(&mut self, bus: &mut Bus, data_ref: u16) {
        if !is_flag_set(&self.cpu_status, ZERO_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if negative flag set
    /// Possible operation HEX: 0x30
    pub fn op_bmi(&mut self, bus: &mut Bus, data_ref: u16) {
        if is_flag_set(&self.cpu_status, NEGATIVE_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if negative flag clear
    /// Possible operation HEX: 0x10
    pub fn op_bpl(&mut self, bus: &mut Bus, data_ref: u16) {
        if !is_flag_set(&self.cpu_status, NEGATIVE_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if overflow flag set
    /// Possible operation HEX: 0x70
    pub fn op_bvs(&mut self, bus: &mut Bus, data_ref: u16) {
        if is_flag_set(&self.cpu_status, OVERFLOW_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }

    /// Branch if overflow flag clear
    /// Possible operation HEX: 0x50
    pub fn op_bvc(&mut self, bus: &mut Bus, data_ref: u16) {
        if !is_flag_set(&self.cpu_status, OVERFLOW_FLAG) {
            let read_data = self.read_8bit(bus, data_ref);
            let relative_displacement = (read_data as i8) as i16;
            self.program_counter = self.program_counter.wrapping_add_signed(relative_displacement)
        }
    }
}

#[test]
fn test_branches() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    const BRANCHABLE_FLAGS: [usize; 4] = [CARRY_FLAG, ZERO_FLAG, NEGATIVE_FLAG, OVERFLOW_FLAG];

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut cpu = Cpu {
        program_counter: 0u16,
        ..Default::default()
    };
    let mut bus = Bus::default();

    let edge_displacements: [i8; 4] = [
        0x00,  // no branch movement
        0x7F,  // largest positive offset (+127)
        -128,  // largest negative offset (-128)
        -1,    // smallest negative offset (-1)
    ];

    for disp in edge_displacements {
        for flag in BRANCHABLE_FLAGS {
            cpu.write_8bit(&mut bus, 0x0000u16, disp as u8);
            cpu.cpu_status = 1 << flag;
            cpu.program_counter = 0x1234;
            call_by_flag(flag, true, &mut cpu, &mut bus);
            assert_eq!(cpu.program_counter, 0x1234u16.wrapping_add_signed(disp as i16));

            cpu.cpu_status = 0;
            cpu.program_counter = 0x1234;
            call_by_flag(flag, true, &mut cpu, &mut bus);
            assert_eq!(cpu.program_counter, 0x1234);
        }
    }

    for _ in 0..10_000 {
        let random_new_pc = rng.random::<u16>();
        cpu.program_counter = random_new_pc;

        let random_cpu_status = rng.random::<u8>();
        cpu.cpu_status = random_cpu_status;

        let random_displacement = rng.random::<i8>();
        check_by_flag(&mut cpu, &mut bus, random_displacement);
    }

    fn check_by_flag(cpu: &mut Cpu, bus: &mut Bus, value: i8) {
        let old_pc = cpu.program_counter;
        cpu.write_8bit(bus, 0x0000u16, value as u8);
        for now_flag in BRANCHABLE_FLAGS {
            if is_flag_set(&cpu.cpu_status, now_flag) {
                call_by_flag(now_flag, true, cpu, bus);
                assert_eq!(cpu.program_counter, old_pc.wrapping_add_signed(value as i16));
                cpu.program_counter = old_pc;

                call_by_flag(now_flag, false, cpu, bus);
                assert_eq!(cpu.program_counter, old_pc);
                cpu.program_counter = old_pc;
            } else {
                call_by_flag(now_flag, false, cpu, bus);
                assert_eq!(cpu.program_counter, old_pc.wrapping_add_signed(value as i16));
                cpu.program_counter = old_pc;

                call_by_flag(now_flag, true, cpu, bus);
                assert_eq!(cpu.program_counter, old_pc);
                cpu.program_counter = old_pc;
            }
        }
    }

    fn call_by_flag(flag_to_call: usize, flag_status: bool, cpu: &mut Cpu, bus: &mut Bus) {
        match (flag_to_call, flag_status) {
            (CARRY_FLAG, true) => cpu.op_bcs(bus, 0x0000),
            (CARRY_FLAG, false) => cpu.op_bcc(bus, 0x0000),
            (ZERO_FLAG, true) => cpu.op_beq(bus, 0x0000),
            (ZERO_FLAG, false) => cpu.op_bne(bus, 0x0000),
            (NEGATIVE_FLAG, true) => cpu.op_bmi(bus, 0x0000),
            (NEGATIVE_FLAG, false) => cpu.op_bpl(bus, 0x0000),
            (OVERFLOW_FLAG, true) => cpu.op_bvs(bus, 0x0000),
            (OVERFLOW_FLAG, false) => cpu.op_bvc(bus, 0x0000),
            _ => unreachable!(),
        }
    }
}
