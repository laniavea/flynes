use std::collections::HashMap;

mod inst_ab;
mod inst_cde;
mod inst_ijlno;
mod inst_prs;
mod inst_t;
mod shared_ops;

#[derive(Debug, Clone)]
pub enum MemoryType {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

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

    pub fn op_type(&self) -> OpType {
        self.op_type.clone()
    }

    pub fn memory_type(&self) -> MemoryType {
        self.memory_type.clone()
    }
}

pub fn init_all_operations() -> HashMap<u8, Operation> {
    let mut operations: HashMap<u8, Operation> = HashMap::with_capacity(256);

    operations.insert(0xA9, Operation::new(0xA9, 2, 2, OpType::OpLDA, MemoryType::Immediate));

    operations
}
