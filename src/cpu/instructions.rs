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
    OpADC,
    OpAdcIm,
    OpAND,
    OpAndIm,
    OpASL,
    OpAslA,
    OpBCC,
    OpBCS,
    OpBEQ,
    OpBIT,
    OpBMI,
    OpBNE,
    OpBPL,
    OpBRK,
    OpBVC,
    OpBVS,
    OpCLC,
    OpCLD,
    OpCLI,
    OpCLV,
    OpCMP,
    OpCmpIm,
    OpCPX,
    OpCpxIm,
    OpCPY,
    OpCpyIm,
    OpDEC,
    OpDEX,
    OpDEY,
    OpEOR,
    OpEorIm,
    OpINC,
    OpINX,
    OpINY,
    OpJMP,
    OpJSR,
    OpLDA,
    OpLdaIm,
    OpLDX,
    OpLdxIm,
    OpLDY,
    OpLdyIm,
    OpLSR,
    OpLsrA,
    OpNOP,
    OpORA,
    OpOraIm,
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
        //TODO: Maybe split by bytes
        match instruction_type {
            OpType::OpADC => self.op_adc(memory_data),
            OpType::OpAdcIm => self.op_adc_im(memory_data),
            OpType::OpAND => self.op_and(memory_data),
            OpType::OpAndIm => self.op_and_im(memory_data),
            OpType::OpASL => self.op_asl(memory_data),
            OpType::OpAslA => self.op_asl_acc(),
            OpType::OpBCC => self.op_bcc(memory_data),
            OpType::OpBCS => self.op_bcs(memory_data),
            OpType::OpBEQ => self.op_beq(memory_data),
            OpType::OpBIT => self.op_bit(memory_data),
            OpType::OpBMI => self.op_bmi(memory_data),
            OpType::OpBNE => self.op_bne(memory_data),
            OpType::OpBPL => self.op_bpl(memory_data),
            OpType::OpBRK => self.op_brk(),
            OpType::OpBVC => self.op_bvc(memory_data),
            OpType::OpBVS => self.op_bvs(memory_data),
            OpType::OpCLC => self.op_clc(),
            OpType::OpCLD => self.op_cld(),
            OpType::OpCLI => self.op_cli(),
            OpType::OpCLV => self.op_clv(),
            OpType::OpCMP => self.op_cmp(memory_data),
            OpType::OpCmpIm => self.op_cmp_im(memory_data),
            OpType::OpCPX => self.op_cpx(memory_data),
            OpType::OpCpxIm => self.op_cpx_im(memory_data),
            OpType::OpCPY => self.op_cpy(memory_data),
            OpType::OpCpyIm => self.op_cpy_im(memory_data),
            OpType::OpDEC => self.op_dec(memory_data),
            OpType::OpDEX => self.op_dex(),
            OpType::OpDEY => self.op_dey(),
            OpType::OpEOR => self.op_eor(memory_data),
            OpType::OpEorIm => self.op_eor_im(memory_data),
            OpType::OpINC => self.op_inc(memory_data),
            OpType::OpINX => self.op_inx(),
            OpType::OpINY => self.op_iny(),
            OpType::OpJMP => self.op_jmp(memory_data),
            OpType::OpJSR => self.op_jsr(memory_data),
            OpType::OpLDA => self.op_lda(memory_data),
            OpType::OpLdaIm => self.op_lda_im(memory_data),
            OpType::OpLDX => self.op_ldx(memory_data),
            OpType::OpLdxIm => self.op_ldx_im(memory_data),
            OpType::OpLDY => self.op_ldy(memory_data),
            OpType::OpLdyIm => self.op_ldy_im(memory_data),
            OpType::OpLSR => self.op_lsr(memory_data),
            OpType::OpLsrA => self.op_lsr_a(),
            OpType::OpNOP => self.op_nop(),
            OpType::OpORA => self.op_ora(memory_data),
            OpType::OpOraIm => self.op_ora_im(memory_data),
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

    // ADC operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#ADC
    // Add accumulator + memory + carry and writes into accumulator
    operations[0x69] = Some(Operation::new(2, 2, OpType::OpAdcIm, MemoryType::Immediate));
    operations[0x65] = Some(Operation::new(2, 3, OpType::OpADC, MemoryType::ZeroPage));
    operations[0x75] = Some(Operation::new(2, 4, OpType::OpADC, MemoryType::ZeroPageX));
    operations[0x6D] = Some(Operation::new(3, 4, OpType::OpADC, MemoryType::Absolute));
    operations[0x7D] = Some(Operation::new(3, 4, OpType::OpADC, MemoryType::AbsoluteX));
    operations[0x79] = Some(Operation::new(3, 4, OpType::OpADC, MemoryType::AbsoluteY));
    operations[0x61] = Some(Operation::new(2, 6, OpType::OpADC, MemoryType::IndirectX));
    operations[0x71] = Some(Operation::new(2, 5, OpType::OpADC, MemoryType::IndirectY));

    // AND operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#AND
    // Perfoms AND operations and writes result to accumulator
    operations[0x29] = Some(Operation::new(2, 2, OpType::OpAndIm, MemoryType::Immediate));
    operations[0x25] = Some(Operation::new(2, 3, OpType::OpAND, MemoryType::ZeroPage));
    operations[0x35] = Some(Operation::new(2, 4, OpType::OpAND, MemoryType::ZeroPageX));
    operations[0x2D] = Some(Operation::new(3, 4, OpType::OpAND, MemoryType::Absolute));
    operations[0x3D] = Some(Operation::new(3, 4, OpType::OpAND, MemoryType::AbsoluteX));
    operations[0x39] = Some(Operation::new(3, 4, OpType::OpAND, MemoryType::AbsoluteY));
    operations[0x21] = Some(Operation::new(2, 6, OpType::OpAND, MemoryType::IndirectX));
    operations[0x31] = Some(Operation::new(2, 5, OpType::OpAND, MemoryType::IndirectY));

    // ASL operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#ASL
    // Perfoms left shift by one bit
    operations[0x0A] = Some(Operation::new(1, 2, OpType::OpAslA, MemoryType::Accumulator));
    operations[0x06] = Some(Operation::new(2, 5, OpType::OpASL, MemoryType::ZeroPage));
    operations[0x16] = Some(Operation::new(2, 6, OpType::OpASL, MemoryType::ZeroPageX));
    operations[0x0E] = Some(Operation::new(3, 6, OpType::OpASL, MemoryType::Absolute));
    operations[0x1E] = Some(Operation::new(3, 7, OpType::OpASL, MemoryType::AbsoluteX));

    // BCC operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BCC
    // Add program counter if carry flag is clear
    operations[0x90] = Some(Operation::new(2,  2, OpType::OpBCC, MemoryType::Relative));

    // BCS operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BCS
    // Add program counter if carry flag is set
    operations[0xB0] = Some(Operation::new(2,  2, OpType::OpBCS, MemoryType::Relative));

    // BEQ operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BEQ
    // Add program counter if zero flag is set
    operations[0xF0] = Some(Operation::new(2,  2, OpType::OpBEQ, MemoryType::Relative));

    // BIT operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BIT
    // Perfoms Acc & data operation then takes 7th and 6th byte of it and transfers to negative and
    // overflow flags
    operations[0x24] = Some(Operation::new(2,  3, OpType::OpBIT, MemoryType::ZeroPage));
    operations[0x2C] = Some(Operation::new(2,  4, OpType::OpBIT, MemoryType::Absolute));

    // BMI operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BMI
    // Add program counter if negative flag is set
    operations[0x30] = Some(Operation::new(2,  2, OpType::OpBMI, MemoryType::Relative));

    // BNE operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BNE
    // Add program counter if zero flag is clear
    operations[0xD0] = Some(Operation::new(2,  2, OpType::OpBNE, MemoryType::Relative));

    // BPL operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BPL
    // Add program counter if negative flag is clear
    operations[0x10] = Some(Operation::new(2,  2, OpType::OpBPL, MemoryType::Relative));

    // BRK operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
    // Creates interrupt
    operations[0x00] = Some(Operation::new(1,  7, OpType::OpBRK, MemoryType::Implied));

    // BVC operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BVC
    // Add program counter if overflow flag is clear
    operations[0x50] = Some(Operation::new(2,  2, OpType::OpBVC, MemoryType::Relative));

    // BVS operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#BVS
    // Add program counter if overflow flag is set
    operations[0x70] = Some(Operation::new(2,  2, OpType::OpBVS, MemoryType::Relative));

    // CLC operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#CLC
    // Sets carry flag to 0
    operations[0x18] = Some(Operation::new(1,  2, OpType::OpCLC, MemoryType::Implied));

    // CLD operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#CLD
    // Sets decimal flag to 0
    operations[0xD8] = Some(Operation::new(1,  2, OpType::OpCLD, MemoryType::Implied));

    // CLI operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#CLI
    // Sets interrupt flag to 0
    operations[0x58] = Some(Operation::new(1,  2, OpType::OpCLI, MemoryType::Implied));

    // CLV operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#CLV
    // Sets overflow flag to 0
    operations[0xB8] = Some(Operation::new(1,  2, OpType::OpCLV, MemoryType::Implied));

    // CMP operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
    // Compare memory to register A and sets flags
    operations[0xC9] = Some(Operation::new(2, 2, OpType::OpCmpIm, MemoryType::Immediate));
    operations[0xC5] = Some(Operation::new(2, 3, OpType::OpCMP, MemoryType::ZeroPage));
    operations[0xD5] = Some(Operation::new(2, 4, OpType::OpCMP, MemoryType::ZeroPageX));
    operations[0xCD] = Some(Operation::new(3, 4, OpType::OpCMP, MemoryType::Absolute));
    operations[0xDD] = Some(Operation::new(3, 4, OpType::OpCMP, MemoryType::AbsoluteX));
    operations[0xD9] = Some(Operation::new(3, 4, OpType::OpCMP, MemoryType::AbsoluteY));
    operations[0xC1] = Some(Operation::new(2, 6, OpType::OpCMP, MemoryType::IndirectX));
    operations[0xD1] = Some(Operation::new(2, 5, OpType::OpCMP, MemoryType::IndirectY));

    // CPX operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#CPX
    // Compare memory to register X and sets flags
    operations[0xE0] = Some(Operation::new(2, 2, OpType::OpCpxIm, MemoryType::Immediate));
    operations[0xE4] = Some(Operation::new(2, 3, OpType::OpCPX, MemoryType::ZeroPage));
    operations[0xEC] = Some(Operation::new(3, 4, OpType::OpCPX, MemoryType::Absolute));

    // CPY operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#CPY
    // Compare memory to register Y and sets flags
    operations[0xC0] = Some(Operation::new(2, 2, OpType::OpCpyIm, MemoryType::Immediate));
    operations[0xC4] = Some(Operation::new(2, 3, OpType::OpCPY, MemoryType::ZeroPage));
    operations[0xCC] = Some(Operation::new(3, 4, OpType::OpCPY, MemoryType::Absolute));

    // DEC operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#DEC
    // Decrement of memory
    operations[0xC6] = Some(Operation::new(2, 5, OpType::OpDEC, MemoryType::ZeroPage));
    operations[0xD6] = Some(Operation::new(2, 6, OpType::OpDEC, MemoryType::ZeroPageX));
    operations[0xCE] = Some(Operation::new(3, 6, OpType::OpDEC, MemoryType::Absolute));
    operations[0xDE] = Some(Operation::new(3, 7, OpType::OpDEC, MemoryType::AbsoluteX));

    // DEX operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#DEX
    // Decrement of X reg
    operations[0xCA] = Some(Operation::new(1, 2, OpType::OpDEX, MemoryType::Implied));

    // DEY operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#DEY
    // Decrement of Y reg
    operations[0x88] = Some(Operation::new(1, 2, OpType::OpDEY, MemoryType::Implied));

    // EOR operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#EOR
    // XOR
    operations[0x49] = Some(Operation::new(2, 2, OpType::OpEorIm, MemoryType::Immediate));
    operations[0x45] = Some(Operation::new(2, 3, OpType::OpEOR, MemoryType::ZeroPage));
    operations[0x55] = Some(Operation::new(2, 4, OpType::OpEOR, MemoryType::ZeroPageX));
    operations[0x4D] = Some(Operation::new(3, 4, OpType::OpEOR, MemoryType::Absolute));
    operations[0x5D] = Some(Operation::new(3, 4, OpType::OpEOR, MemoryType::AbsoluteX));
    operations[0x59] = Some(Operation::new(3, 4, OpType::OpEOR, MemoryType::AbsoluteY));
    operations[0x41] = Some(Operation::new(2, 6, OpType::OpEOR, MemoryType::IndirectX));
    operations[0x51] = Some(Operation::new(2, 5, OpType::OpEOR, MemoryType::IndirectY));

    // INC operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#INC
    // Increments memory
    operations[0xE6] = Some(Operation::new(2, 5, OpType::OpINC, MemoryType::ZeroPage));
    operations[0xF6] = Some(Operation::new(2, 6, OpType::OpINC, MemoryType::ZeroPageX));
    operations[0xEE] = Some(Operation::new(3, 6, OpType::OpINC, MemoryType::Absolute));
    operations[0xFE] = Some(Operation::new(3, 7, OpType::OpINC, MemoryType::AbsoluteX));

    // INX operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
    // Increments X register
    operations[0xE8] = Some(Operation::new(1, 2, OpType::OpINX, MemoryType::Implied));

    // INY operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#INY
    // Increments Y register
    operations[0xC8] = Some(Operation::new(1, 2, OpType::OpINY, MemoryType::Implied));

    // JMP operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
    // Jumps to another location to run code
    operations[0x4C] = Some(Operation::new(3, 3, OpType::OpJMP, MemoryType::Absolute));
    operations[0x6C] = Some(Operation::new(3, 5, OpType::OpJMP, MemoryType::Indirect));

    // JSR operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#JSR
    // Pushed correct address to the stack and setes program counter to target memory
    operations[0x20] = Some(Operation::new(3, 6, OpType::OpJSR, MemoryType::Absolute));

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
    // Perfoms logical shift right
    operations[0x4A] = Some(Operation::new(1, 2, OpType::OpLsrA, MemoryType::Accumulator));
    operations[0x46] = Some(Operation::new(2, 5, OpType::OpLSR, MemoryType::ZeroPage));
    operations[0x56] = Some(Operation::new(2, 6, OpType::OpLSR, MemoryType::ZeroPageX));
    operations[0x4E] = Some(Operation::new(3, 6, OpType::OpLSR, MemoryType::Absolute));
    operations[0x5E] = Some(Operation::new(3, 7, OpType::OpLSR, MemoryType::AbsoluteX));

    // NOP operation - https://www.nesdev.org/obelisk-6502-guide/reference.html#NOP
    // No operation, just inc of program counter and cpu cycles
    operations[0xEA] = Some(Operation::new(1, 2, OpType::OpNOP, MemoryType::Implied));

    // ORA operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#ORA
    // Inclusive OR between accumulator and other memory
    operations[0x09] = Some(Operation::new(2, 2, OpType::OpOraIm, MemoryType::Immediate));
    operations[0x05] = Some(Operation::new(2, 3, OpType::OpORA, MemoryType::ZeroPage));
    operations[0x15] = Some(Operation::new(2, 4, OpType::OpORA, MemoryType::ZeroPageX));
    operations[0x0D] = Some(Operation::new(3, 4, OpType::OpORA, MemoryType::Absolute));
    operations[0x1D] = Some(Operation::new(3, 4, OpType::OpORA, MemoryType::AbsoluteX));
    operations[0x19] = Some(Operation::new(3, 4, OpType::OpORA, MemoryType::AbsoluteY));
    operations[0x01] = Some(Operation::new(2, 6, OpType::OpORA, MemoryType::IndirectX));
    operations[0x11] = Some(Operation::new(2, 5, OpType::OpORA, MemoryType::IndirectY));

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
