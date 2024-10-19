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
    OpJMP,
    OpLDA,
    OpLdaIm,
    OpLDX,
    OpLdxIm,
    OpLDY,
    OpLdyIm,
    OpLSR,
    OpLsrA,
    OpPHA,
    OpPHP,
    OpPLA,
    OpPLP,
    OpROL,
    OpRolA,
    OpROR,
    OpRorA,
    OpRTI,
    OpRTS,
    OpSBC,
    OpSBCIm,
    OpSEC,
    OpSED,
    OpSEI,
    OpSTA,
    OpSTX,
    OpSTY,
    OpTAX,
    OpTAY,
    OpTSX,
    OpTXA,
    OpTXS,
    OpTYA,
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
            OpType::OpJMP => self.op_jmp(memory_data),
            OpType::OpLDA => self.op_lda(memory_data),
            OpType::OpLdaIm => self.op_lda_im(memory_data),
            OpType::OpLDX => self.op_ldx(memory_data),
            OpType::OpLdxIm => self.op_ldx_im(memory_data),
            OpType::OpLDY => self.op_ldy(memory_data),
            OpType::OpLdyIm => self.op_ldy_im(memory_data),
            OpType::OpLSR => self.op_lsr(memory_data),
            OpType::OpLsrA => self.op_lsr_a(),
            OpType::OpPHA => self.op_pha(),
            OpType::OpPHP => self.op_php(),
            OpType::OpPLA => self.op_pla(),
            OpType::OpPLP => self.op_plp(),
            OpType::OpROL => self.op_rol(memory_data),
            OpType::OpRolA => self.op_rol_a(),
            OpType::OpROR => self.op_ror(memory_data),
            OpType::OpRorA => self.op_ror_a(),
            OpType::OpRTI => self.op_rti(),
            OpType::OpRTS => self.op_rts(),
            OpType::OpSBC => self.op_sbc(memory_data),
            OpType::OpSBCIm => self.op_sbc_im(memory_data),
            OpType::OpSEC => self.op_sec(),
            OpType::OpSED => self.op_sed(),
            OpType::OpSEI => self.op_sei(),
            OpType::OpSTA => self.op_sta(memory_data),
            OpType::OpSTX => self.op_stx(memory_data),
            OpType::OpSTY => self.op_sty(memory_data),
            OpType::OpTAX => self.op_tax(),
            OpType::OpTAY => self.op_tay(),
            OpType::OpTSX => self.op_tsx(),
            OpType::OpTXA => self.op_txa(),
            OpType::OpTXS => self.op_txs(),
            OpType::OpTYA => self.op_tya(),
        }
    }
}

pub fn init_all_operations() -> [Option<Operation>; 256] {
    let mut operations: [Option<Operation>; 256] = [None; 256];

    // JMP operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    operations[0x4C] = Some(Operation::new(3, 3, OpType::OpJMP, MemoryType::Absolute));
    operations[0x6C] = Some(Operation::new(3, 5, OpType::OpJMP, MemoryType::Indirect));

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

    // PHA operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
    // Pushes register A to the stack
    operations[0x48] = Some(Operation::new(1, 3, OpType::OpPHA, MemoryType::Implied));

    // PHP operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#PHP
    // Pushes cpu status to the stack
    operations[0x08] = Some(Operation::new(1, 3, OpType::OpPHP, MemoryType::Implied));

    // PLA operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#PLA
    // Sets register A from the stack
    operations[0x68] = Some(Operation::new(1, 4, OpType::OpPLA, MemoryType::Implied));

    // PLP operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#PLP
    // Sets processor status from the stack
    operations[0x28] = Some(Operation::new(1, 4, OpType::OpPLP, MemoryType::Implied));

    // LSR operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#LSR
    // Perfomas logical shift right
    operations[0x4A] = Some(Operation::new(1, 2, OpType::OpLsrA, MemoryType::Accumulator));
    operations[0x46] = Some(Operation::new(2, 5, OpType::OpLSR, MemoryType::ZeroPage));
    operations[0x56] = Some(Operation::new(2, 6, OpType::OpLSR, MemoryType::ZeroPageX));
    operations[0x4E] = Some(Operation::new(3, 6, OpType::OpLSR, MemoryType::Absolute));
    operations[0x5E] = Some(Operation::new(3, 7, OpType::OpLSR, MemoryType::AbsoluteX));

    // ROL instructions - https://www.nesdev.org/obelisk-6502-guide/reference.html#ROL
    // Left shift with carry manipulations
    operations[0x2A] = Some(Operation::new(1, 2, OpType::OpRolA, MemoryType::Accumulator));
    operations[0x26] = Some(Operation::new(2, 5, OpType::OpROL, MemoryType::ZeroPage));
    operations[0x36] = Some(Operation::new(2, 6, OpType::OpROL, MemoryType::ZeroPageX));
    operations[0x2E] = Some(Operation::new(3, 6, OpType::OpROL, MemoryType::Absolute));
    operations[0x3E] = Some(Operation::new(3, 7, OpType::OpROL, MemoryType::AbsoluteX));
    
    // ROR instructions - https://www.nesdev.org/obelisk-6502-guide/reference.html#ROR
    // Right shift with carry manipulations
    operations[0x6A] = Some(Operation::new(1, 2, OpType::OpRorA, MemoryType::Accumulator));
    operations[0x66] = Some(Operation::new(2, 5, OpType::OpROR, MemoryType::ZeroPage));
    operations[0x76] = Some(Operation::new(2, 6, OpType::OpROR, MemoryType::ZeroPageX));
    operations[0x6E] = Some(Operation::new(3, 6, OpType::OpROR, MemoryType::Absolute));
    operations[0x7E] = Some(Operation::new(3, 7, OpType::OpROR, MemoryType::AbsoluteX));

    // RTI operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#RTI
    // Pulls processor flag from the stack and then pull porgram counter from it
    operations[0x40] = Some(Operation::new(1, 6, OpType::OpRTI, MemoryType::Implied));

    // RTS operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#RTS
    // Pulls program counter (minus one) from stack
    operations[0x60] = Some(Operation::new(1, 6, OpType::OpRTS, MemoryType::Implied));

    // SBC operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#SBC
    // Substructs content of a memory locations to the accumulator
    operations[0xE9] = Some(Operation::new(2, 2, OpType::OpSBCIm, MemoryType::Immediate));
    operations[0xE5] = Some(Operation::new(2, 3, OpType::OpSBC, MemoryType::ZeroPage));
    operations[0xF5] = Some(Operation::new(2, 4, OpType::OpSBC, MemoryType::ZeroPageX));
    operations[0xED] = Some(Operation::new(3, 4, OpType::OpSBC, MemoryType::Absolute));
    operations[0xFD] = Some(Operation::new(3, 4, OpType::OpSBC, MemoryType::AbsoluteX));
    operations[0xF9] = Some(Operation::new(3, 4, OpType::OpSBC, MemoryType::AbsoluteY));
    operations[0xE1] = Some(Operation::new(2, 6, OpType::OpSBC, MemoryType::IndirectX));
    operations[0xF1] = Some(Operation::new(2, 5, OpType::OpSBC, MemoryType::IndirectY));

    // SEC operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#SEC
    // Sets carry flag to one
    operations[0x38] = Some(Operation::new(1, 2, OpType::OpSEC, MemoryType::Implied));

    // SED operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#SED
    // Sets decimal flag to one
    operations[0xF8] = Some(Operation::new(1, 2, OpType::OpSED, MemoryType::Implied));

    // SEI operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#SEI
    // Sets interrupt disable flag to one
    operations[0x78] = Some(Operation::new(1, 2, OpType::OpSEI, MemoryType::Implied));

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

    // TAX operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
    // Content from A reg to X reg
    operations[0xAA] = Some(Operation::new(1, 2, OpType::OpTAX, MemoryType::Implied));

    // TAY operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TAY
    // Content from A reg to Y reg
    operations[0xA8] = Some(Operation::new(1, 2, OpType::OpTAY, MemoryType::Implied));

    // TSX operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TSX
    // Copies content from stack reg to X
    operations[0xBA] = Some(Operation::new(1, 2, OpType::OpTSX, MemoryType::Implied));
    
    // TXA operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TXA
    // Copies content from reg X to reg A
    operations[0x8A] = Some(Operation::new(1, 2, OpType::OpTXA, MemoryType::Implied));

    // TXS operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TXS
    // Copies content from reg X to stack reg
    operations[0x9A] = Some(Operation::new(1, 2, OpType::OpTXS, MemoryType::Implied));

    // TYA operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#TYA
    // Copies content from Y reg to A reg
    operations[0x98] = Some(Operation::new(1, 2, OpType::OpTYA, MemoryType::Implied));

    info!("Operations array created with {} elements", operations.iter().filter(|val| val.is_some()).count());

    operations
}
