use std::collections::HashMap;

mod inst_ab;
mod inst_cde;
mod inst_ijlno;
mod inst_prs;
mod inst_t;

enum MemoryType {
    Immediate,
    ZeroPage,
    ZeroPageX,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
}

enum OpType {
    OpLDA,
}

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
}

pub fn init_all_operations() -> HashMap<u8, Operation> {
    let mut operations: HashMap<u8, Operation> = HashMap::with_capacity(256);

    operations.insert(0xA9, Operation::new(0xA9, 2, 2, OpType::OpLDA, MemoryType::Immediate));

    operations
}
