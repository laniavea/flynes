use crate::cpu::Cpu;
use crate::bus::Bus;

impl Cpu {
    /// Jump operation. Sets PC to specified address
    /// Possible operation HEX: 0x4C, 0x6C
    pub fn op_jmp(&mut self, data: u16) {
        self.program_counter = data;
    }

    /// Jump to subroutine. Pushes address (-1) of the return point on the stack and sets the PC to the target address
    /// Possible operation HEX: 0x20
    pub fn op_jsr(&mut self, bus: &mut Bus, data: u16) {
        self.program_counter = self.program_counter.wrapping_sub(1);
        bus.memory_mut().stack_push_16bit(self.program_counter, &mut self.stack_pointer);
        self.program_counter = data;
    }

    /// Return from subroutine. Pulls the PC (-1) from the stack and sets it as actual PC.
    /// Possible operation HEX: 0x60
    pub fn op_rts(&mut self, bus: &Bus) {
        self.program_counter = bus.memory().stack_pull_16bit(&mut self.stack_pointer);
        self.program_counter = self.program_counter.wrapping_add(1);
    }
}

// #[test]
// fn test_jump_calls() {
//     use rand::{SeedableRng, Rng};
//     use rand::rngs::StdRng;
//
//     let mut cpu = Cpu {
//         stack_pointer: 0b0000_0000,
//         program_counter: 0u16,
//         ..Default::default()
//     };
//
//     let mut rng: StdRng = StdRng::seed_from_u64(42);
//     let mut mem: Memory = Memory::default();
//
//     for _ in 0..1000 {
//         let random_new_pc = rng.random::<u16>();
//         let random_sp = rng.random::<u8>();
//         cpu.stack_pointer = random_sp;
//         let random_depth = rng.random::<u8>() / 2;
//
//         cpu.op_jmp(random_new_pc);
//         assert_eq!(cpu.program_counter, random_new_pc);
//
//         let mut random_pcs: Vec<u16> = Vec::new();
//         let mut stack_growth_rate: u8 = 0;
//
//         for _ in 0..random_depth {
//             let new_random_pc = rng.random::<u16>();
//             cpu.op_jsr(new_random_pc, &mut mem);
//             random_pcs.push(new_random_pc);
//             stack_growth_rate = stack_growth_rate.wrapping_add(2);
//             assert_eq!(cpu.stack_pointer, random_sp.wrapping_sub(stack_growth_rate));
//         }
//
//         for now_pc in random_pcs.iter().rev() {
//             assert_eq!(cpu.program_counter, *now_pc);
//             cpu.op_rts(&mem);
//             stack_growth_rate = stack_growth_rate.wrapping_sub(2);
//             assert_eq!(cpu.stack_pointer, random_sp.wrapping_sub(stack_growth_rate));
//         }
//
//         assert_eq!(cpu.stack_pointer, random_sp);
//         assert_eq!(cpu.program_counter, random_new_pc);
//     }
// }
