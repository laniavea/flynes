use crate::cpu::Cpu;

#[derive(Debug, Clone)]
pub enum MemoryType {
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
    pub fn read_memory(&self, pointer: u16, memory_type: MemoryType) -> u8 {
        match memory_type {
            MemoryType::Immediate => {
                pointer as u8
            },
            MemoryType::ZeroPage => {
                self.memory[pointer as usize]
            },
            MemoryType::ZeroPageX => {
                self.memory[pointer as usize + self.reg_x as usize]
            },
            MemoryType::ZeroPageY => {
                self.memory[pointer as usize + self.reg_y as usize]
            },
            MemoryType::Absolute => {
                self.memory[pointer as usize]
            },
            MemoryType::AbsoluteX => {
                self.memory[pointer as usize + self.reg_x as usize]
            },
            MemoryType::AbsoluteY => {
                self.memory[pointer as usize + self.reg_y as usize]
            },
            MemoryType::IndirectX => {
                let zero_page_add = (self.reg_x + pointer as u8) as usize;
                let abs_add = self.memory[zero_page_add] as usize + ((self.memory[zero_page_add + 1] as usize) << 8);
                self.memory[abs_add]
            },
            MemoryType::IndirectY => {
                let zero_page_add = (self.reg_y + pointer as u8) as usize;
                let abs_add = self.memory[zero_page_add] as usize + ((self.memory[zero_page_add + 1] as usize) << 8);
                self.memory[abs_add]
            },
            _ => {
                unimplemented!();
            },
        }
    }
}
