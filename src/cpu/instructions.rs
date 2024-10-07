use std::collections::HashMap;

use log::{debug, info};

use crate::cpu::Cpu;
use crate::cpu::memory::MemoryType;

mod inst_ab;
mod inst_cde;
mod inst_ijlno;
mod inst_prs;
mod inst_t;
mod shared_ops;

#[derive(Debug, Clone)]
pub enum OpType {
    OpLDA,
}

#[derive(Debug, Clone)]
pub struct Operation {
   op_id: u8,
   bytes: u8,
   cycles: u8,
   op_type: OpType,
   memory_type: MemoryType,
}

impl Operation {
    pub fn new(op_id: u8, bytes: u8, cycles: u8, op_type: OpType, memory_type: MemoryType) -> Operation {
        Self {
            op_id,
            bytes,
            cycles,
            op_type,
            memory_type,
        }
    }

    pub fn op_id(&self) -> u8 {
        self.op_id
    }

    pub fn bytes(&self) -> u8 {
        self.bytes
    }

    pub fn cycles(&self) -> u8 {
        self.cycles
    }

    pub fn op_type(&self) -> &OpType {
        &self.op_type
    }

    pub fn memory_type(&self) -> &MemoryType {
        &self.memory_type
    }
}

impl Cpu {
    pub fn do_insturction(&mut self, data: u8, instruction_type: OpType) {
        match instruction_type {
            OpType::OpLDA => {
                self.op_lda(data);
            },
        }
    }
}

pub fn init_all_operations() -> HashMap<u8, Operation> {
    let mut operations: HashMap<u8, Operation> = HashMap::with_capacity(256);

    // LDA operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
    operations.insert(0xA9, Operation::new(0xA9, 2, 2, OpType::OpLDA, MemoryType::Immediate));
    operations.insert(0xA5, Operation::new(0xA5, 2, 3, OpType::OpLDA, MemoryType::ZeroPage));
    operations.insert(0xB5, Operation::new(0xB5, 2, 4, OpType::OpLDA, MemoryType::ZeroPageX));
    operations.insert(0xAD, Operation::new(0xAD, 3, 4, OpType::OpLDA, MemoryType::Absolute));
    operations.insert(0xBD, Operation::new(0xBD, 3, 4, OpType::OpLDA, MemoryType::AbsoluteX));
    operations.insert(0xB9, Operation::new(0xB9, 3, 4, OpType::OpLDA, MemoryType::AbsoluteY));

    info!("Operations hashmap created with {} elements", operations.len());

    operations
}
