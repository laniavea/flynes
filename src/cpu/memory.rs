use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MemoryType {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

// Memory Structure:
// $0000 - $00FF -> ZeroPage (can be accessed with fewer bytes and cycles than other modules)
// $0100 - $01FF -> Stack (can be start anywhere but usually starts from 01FF and grows downward)

impl Cpu {
    /// Function to read data from memory in different modes from selected pointer.
    /// The meaning of pointer changes based on memory_type.
    /// more info at https://www.nesdev.org/obelisk-6502-guide/addressing.html
    pub fn ref_to_memory_by_address(&mut self, pointer: u16, memory_type: MemoryType) -> u16 {
        match memory_type {
            MemoryType::Accumulator => {
                0
            },
            MemoryType::Immediate => {
                pointer
            },
            MemoryType::ZeroPage => {
                pointer
            },
            MemoryType::ZeroPageX => {
                (pointer as u8 + self.reg_x) as u16
            },
            MemoryType::ZeroPageY => {
                (pointer as u8 + self.reg_y) as u16
            },
            MemoryType::Absolute => {
                pointer
            },
            MemoryType::AbsoluteX => {
                pointer + self.reg_x as u16
            },
            MemoryType::AbsoluteY => {
                pointer + self.reg_y as u16
            },
            MemoryType::IndirectX => {
                let zero_page_add = (self.reg_x + pointer as u8) as usize;
                self.memory[zero_page_add] as u16 + ((self.memory[zero_page_add + 1] as u16) << 8)
            },
            MemoryType::IndirectY => {
                let zero_page_add = (self.reg_y + pointer as u8) as usize;
                self.memory[zero_page_add] as u16 + ((self.memory[zero_page_add + 1] as u16) << 8)
            },
            _ => {
                unimplemented!();
            },
        }
    }
}
