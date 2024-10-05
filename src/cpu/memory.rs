use crate::cpu::Cpu;
use crate::cpu::instructions::MemoryType;

impl Cpu {
    pub fn read_memory(&self, pointer: u16, memory_type: MemoryType) -> u8 {
        match memory_type {
            MemoryType::Immediate => {
                pointer as u8
            },
            _ => {
                0
            },
        }
    }
}
