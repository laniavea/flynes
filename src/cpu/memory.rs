use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum MemoryType {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

const BASE_STACK_POINTER: usize = 0x01FF;

// Memory Structure:
// $0000 - $00FF -> ZeroPage (can be accessed with fewer bytes and cycles than other modules)
// $0100 - $01FF -> Stack (can be start anywhere but usually starts from 01FF and grows downward)
// $0200 - $07FF -> RAM
// $0800 - $1FFF -> Mirrors for $0000 - $07FF
// $2000 - $2007 -> IO registers
// $2008 - $3FFF -> Mirrors for $2000 - $2007
// $4000 - $401F -> IO registers
// $4020 - $5FFF -> Expansion ROM
// $6000 - $7FFF -> SRAM
// $8000 - $BFFF -> PRG-ROM LB
// $C000 - $FFFF -> PRG-ROM UB
// More in https://www.nesdev.org/NESDoc.pdf

impl Cpu {
    //TODO: Do a write mem function
    pub fn _write_mem(&mut self, pointer: u16, data: u8) {
    }

    /// Function to pop data from stack by stack pointer
    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.memory[BASE_STACK_POINTER - (self.stack_pointer as usize)]
    }

    /// Function to push data to stack by stack pointer
    pub fn stack_push(&mut self, value: u8) {
        self.memory[BASE_STACK_POINTER - self.stack_pointer as usize] = value;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }
}

impl Cpu {
    /// Function to read data from memory in different modes from selected pointer.
    /// The meaning of pointer changes based on memory_type.
    /// more info at https://www.nesdev.org/obelisk-6502-guide/addressing.html
    pub fn ref_to_memory_by_address(&self, pointer: u16, memory_type: MemoryType) -> u16 {
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
            MemoryType::Indirect => {
                self.memory[pointer as usize] as u16 + ((self.memory[pointer as usize +1] as u16) << 8)
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
