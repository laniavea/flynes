use log::info;

use crate::cpu::Cpu;
use crate::cpu::memory::MemoryType;

mod inst_ab;
mod inst_cde;
mod inst_ijlno;
mod inst_prs;
mod inst_t;
mod shared_ops;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpType {
    OpLDA,
    OpLdaIm,
    OpLDX,
    OpLdxIm,
    OpLDY,
    OpLdyIm,
    OpLSR,
    OpLsrA,
    OpSTA,
    OpSTX,
    OpSTY,
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
   bytes: u8,
   _cycles: u8,
   op_type: OpType,
   memory_type: MemoryType,
}

impl Operation {
    pub fn new(bytes: u8, cycles: u8, op_type: OpType, memory_type: MemoryType) -> Operation {
        Self {
            bytes,
            _cycles: cycles,
            op_type,
            memory_type,
        }
    }

    pub fn bytes(&self) -> u8 {
        self.bytes
    }

    pub fn _cycles(&self) -> u8 {
        self._cycles
    }

    pub fn op_type(&self) -> OpType {
        self.op_type
    }

    pub fn memory_type(&self) -> MemoryType {
        self.memory_type
    }
}

impl Cpu {
    pub fn do_insturction(&mut self, memory_data: u16, instruction_type: OpType) {
        match instruction_type {
            OpType::OpLDA => self.op_lda(memory_data),
            OpType::OpLdaIm => self.op_lda_im(memory_data),
            OpType::OpLDX => self.op_ldx(memory_data),
            OpType::OpLdxIm => self.op_ldx_im(memory_data),
            OpType::OpLDY => self.op_ldy(memory_data),
            OpType::OpLdyIm => self.op_ldy_im(memory_data),
            OpType::OpLSR => self.op_lsr(memory_data),
            OpType::OpLsrA => self.op_lsr_a(),
            OpType::OpSTA => self.op_sta(memory_data),
            OpType::OpSTX => self.op_stx(memory_data),
            OpType::OpSTY => self.op_sty(memory_data),
        }
    }
}

pub fn init_all_operations() -> [Option<Operation>; 256] {
    let mut operations: [Option<Operation>; 256] = [None; 256];

    // LDA operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    // Append data to register A
    operations[0xA9] = Some(Operation::new(2, 2, OpType::OpLdaIm, MemoryType::Immediate));
    operations[0xA5] = Some(Operation::new(2, 3, OpType::OpLDA, MemoryType::ZeroPage));
    operations[0xB5] = Some(Operation::new(2, 4, OpType::OpLDA, MemoryType::ZeroPageX));
    operations[0xAD] = Some(Operation::new(3, 4, OpType::OpLDA, MemoryType::Absolute));
    operations[0xBD] = Some(Operation::new(3, 4, OpType::OpLDA, MemoryType::AbsoluteX));
    operations[0xB9] = Some(Operation::new(3, 4, OpType::OpLDA, MemoryType::AbsoluteY));
    operations[0xA1] = Some(Operation::new(2, 6, OpType::OpLDA, MemoryType::IndirectX));
    operations[0xB1] = Some(Operation::new(2, 5, OpType::OpLDA, MemoryType::IndirectY));

    // LDX operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDX
    // Append data to register X
    operations[0xA2] = Some(Operation::new(2, 2, OpType::OpLdxIm, MemoryType::Immediate));
    operations[0xA6] = Some(Operation::new(2, 3, OpType::OpLDX, MemoryType::ZeroPage));
    operations[0xB6] = Some(Operation::new(2, 4, OpType::OpLDX, MemoryType::ZeroPageY));
    operations[0xAE] = Some(Operation::new(3, 4, OpType::OpLDX, MemoryType::Absolute));
    operations[0xBE] = Some(Operation::new(3, 4, OpType::OpLDX, MemoryType::AbsoluteY));

    // LDY operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDY
    // Append data to register Y
    operations[0xA0] = Some(Operation::new(2, 2, OpType::OpLdyIm, MemoryType::Immediate));
    operations[0xA4] = Some(Operation::new(2, 3, OpType::OpLDY, MemoryType::ZeroPage));
    operations[0xB4] = Some(Operation::new(2, 4, OpType::OpLDY, MemoryType::ZeroPageX));
    operations[0xAC] = Some(Operation::new(3, 4, OpType::OpLDY, MemoryType::Absolute));
    operations[0xBC] = Some(Operation::new(3, 4, OpType::OpLDY, MemoryType::AbsoluteX));

    // LSR operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LSR
    // Perfomas logical shift right
    operations[0x4A] = Some(Operation::new(1, 2, OpType::OpLsrA, MemoryType::Accumulator));
    operations[0x46] = Some(Operation::new(2, 5, OpType::OpLSR, MemoryType::ZeroPage));
    operations[0x56] = Some(Operation::new(2, 6, OpType::OpLSR, MemoryType::ZeroPageX));
    operations[0x4E] = Some(Operation::new(3, 6, OpType::OpLSR, MemoryType::Absolute));
    operations[0x5E] = Some(Operation::new(3, 7, OpType::OpLSR, MemoryType::AbsoluteX));

    // STA operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#STA
    // Append data from register A to memory
    operations[0x85] = Some(Operation::new(2, 3, OpType::OpSTA, MemoryType::ZeroPage));
    operations[0x95] = Some(Operation::new(2, 4, OpType::OpSTA, MemoryType::ZeroPageX));
    operations[0x8D] = Some(Operation::new(3, 4, OpType::OpSTA, MemoryType::Absolute));
    operations[0x9D] = Some(Operation::new(3, 5, OpType::OpSTA, MemoryType::AbsoluteX));
    operations[0x99] = Some(Operation::new(3, 5, OpType::OpSTA, MemoryType::AbsoluteY));
    operations[0x81] = Some(Operation::new(2, 6, OpType::OpSTA, MemoryType::IndirectX));
    operations[0x91] = Some(Operation::new(2, 6, OpType::OpSTA, MemoryType::IndirectY));

    // STX operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#STX
    // Append data from register X to memory
    operations[0x86] = Some(Operation::new(2, 3, OpType::OpSTX, MemoryType::ZeroPage));
    operations[0x96] = Some(Operation::new(2, 4, OpType::OpSTX, MemoryType::ZeroPageY));
    operations[0x8E] = Some(Operation::new(3, 4, OpType::OpSTX, MemoryType::Absolute));

    // STY operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#STY
    // Append data from register Y to memory
    operations[0x84] = Some(Operation::new(2, 3, OpType::OpSTY, MemoryType::ZeroPage));
    operations[0x94] = Some(Operation::new(2, 4, OpType::OpSTY, MemoryType::ZeroPageY));
    operations[0x8C] = Some(Operation::new(3, 4, OpType::OpSTY, MemoryType::Absolute));

    info!("Operations array created with {} elements", operations.iter().filter(|val| val.is_some()).count());

    operations
}
