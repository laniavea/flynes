use crate::cpu::Cpu;
use crate::memory::{Memory, MemoryType};

mod load_store;
mod register_transfer;
mod stack_operations;
mod logical;
mod arithmetic;
mod increment_decrement;
mod shifts;
mod jumps_calls;
mod branches;
mod status_flag_changes;
mod system_functions;
mod unofficial_combined;
mod unofficial_rmw;
mod unofficial_other;

mod shared_ops;

const NO_OP: Operation = Operation {
    cycles: 0,
    cycles_pgcr: 0,
    memory_type: MemoryType::Implied,
    op_name: CPUInstByte::NoOp,
};

#[derive(Debug, Clone, Copy)]
pub enum CPUInstByte {
    One(Inst1Byte),
    Two(Inst2Byte),
    Three(Inst3Byte),
    NoOp,
}

impl CPUInstByte {
    pub fn as_digit(&self) -> usize {
        match self {
            CPUInstByte::One(_) => 1,
            CPUInstByte::Two(_) => 2,
            CPUInstByte::Three(_) => 3,
            CPUInstByte::NoOp => 0,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Inst1Byte {
    TAXop,
    TAYop,
    TXAop,
    TYAop,
    TSXop,
    TXSop,
    PHAop,
    PHPop,
    PLAop,
    PLPop,
    INXop,
    INYop,
    DEXop,
    DEYop,
    ASLop,
    LSRop,
    ROLop,
    RORop,
    RTSop,
    CLCop,
    CLDop,
    CLIop,
    CLVop,
    SECop,
    SEDop,
    SEIop,
    BRKop,
    NOPop,
    RTIop,
    STPop,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Inst2Byte {
    LDAop,
    LDXop,
    LDYop,
    STAop,
    STXop,
    STYop,
    ANDop,
    EORop,
    ORAop,
    BITop,
    ADCop,
    SBCop,
    CMPop,
    CPXop,
    CPYop,
    INCop,
    DECop,
    ASLop,
    LSRop,
    ROLop,
    RORop,
    BCCop,
    BCSop,
    BEQop,
    BMIop,
    BNEop,
    BPLop,
    BVCop,
    BVSop,
    ALRop,
    ANCop,
    ARRop,
    AXSop,
    LAXop,
    SAXop,
    DCPop,
    ISCop,
    RLAop,
    RRAop,
    SLOop,
    SREop,
    NOPop,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Inst3Byte {
    LDAop,
    LDXop,
    LDYop,
    STAop,
    STXop,
    STYop,
    ANDop,
    EORop,
    ORAop,
    BITop,
    ADCop,
    SBCop,
    CMPop,
    CPXop,
    CPYop,
    INCop,
    DECop,
    ASLop,
    LSRop,
    ROLop,
    RORop,
    JMPop,
    JSRop,
    LAXop,
    SAXop,
    DCPop,
    ISCop,
    RLAop,
    RRAop,
    SLOop,
    SREop,
    SHXop,
    SHYop,
    NOPop,
}

impl Cpu {
    pub fn execute_inst_1_byte(&mut self, now_inst: Inst1Byte, memory: &mut Memory) {
        match now_inst {
            Inst1Byte::TAXop => self.op_tax(),
            Inst1Byte::TAYop => self.op_tay(),
            Inst1Byte::TXAop => self.op_txa(),
            Inst1Byte::TYAop => self.op_tya(),
            Inst1Byte::TSXop => self.op_tsx(),
            Inst1Byte::TXSop => self.op_txs(),
            Inst1Byte::PHAop => self.op_pha(memory),
            Inst1Byte::PHPop => self.op_php(memory),
            Inst1Byte::PLAop => self.op_pla(memory),
            Inst1Byte::PLPop => self.op_plp(memory),
            Inst1Byte::INXop => self.op_inx(),
            Inst1Byte::INYop => self.op_iny(),
            Inst1Byte::DEXop => self.op_dex(),
            Inst1Byte::DEYop => self.op_dey(),
            Inst1Byte::ASLop => self.op_asl_acc(),
            Inst1Byte::LSRop => self.op_lsr_acc(),
            Inst1Byte::ROLop => self.op_rol_acc(),
            Inst1Byte::RORop => self.op_ror_acc(),
            Inst1Byte::RTSop => self.op_rts(memory),
            Inst1Byte::CLCop => self.op_clc(),
            Inst1Byte::CLDop => self.op_cld(),
            Inst1Byte::CLIop => self.op_cli(),
            Inst1Byte::CLVop => self.op_clv(),
            Inst1Byte::SECop => self.op_sec(),
            Inst1Byte::SEDop => self.op_sed(),
            Inst1Byte::SEIop => self.op_sei(),
            Inst1Byte::BRKop => self.op_brk(memory),
            Inst1Byte::NOPop => self.op_nop(),
            Inst1Byte::RTIop => self.op_rti(memory),
            Inst1Byte::STPop => self.op_stp(),
        }
    }

    pub fn execute_inst_2_byte(&mut self, now_inst: Inst2Byte, data_ref: &mut u8) {
        match now_inst {
            Inst2Byte::LDAop => self.op_lda(data_ref),
            Inst2Byte::LDXop => self.op_ldx(data_ref),
            Inst2Byte::LDYop => self.op_ldy(data_ref),
            Inst2Byte::STAop => self.op_sta(data_ref),
            Inst2Byte::STXop => self.op_stx(data_ref),
            Inst2Byte::STYop => self.op_sty(data_ref),
            Inst2Byte::ANDop => self.op_and(data_ref),
            Inst2Byte::EORop => self.op_eor(data_ref),
            Inst2Byte::ORAop => self.op_ora(data_ref),
            Inst2Byte::BITop => self.op_bit(data_ref),
            Inst2Byte::ADCop => self.op_adc(data_ref),
            Inst2Byte::SBCop => self.op_sbc(data_ref),
            Inst2Byte::CMPop => self.op_cmp(data_ref),
            Inst2Byte::CPXop => self.op_cpx(data_ref),
            Inst2Byte::CPYop => self.op_cpy(data_ref),
            Inst2Byte::INCop => self.op_inc(data_ref),
            Inst2Byte::DECop => self.op_dec(data_ref),
            Inst2Byte::ASLop => self.op_asl(data_ref),
            Inst2Byte::LSRop => self.op_lsr(data_ref),
            Inst2Byte::ROLop => self.op_rol(data_ref),
            Inst2Byte::RORop => self.op_ror(data_ref),
            Inst2Byte::BCCop => self.op_bcc(data_ref),
            Inst2Byte::BCSop => self.op_bcs(data_ref),
            Inst2Byte::BEQop => self.op_beq(data_ref),
            Inst2Byte::BMIop => self.op_bmi(data_ref),
            Inst2Byte::BNEop => self.op_bne(data_ref),
            Inst2Byte::BPLop => self.op_bpl(data_ref),
            Inst2Byte::BVCop => self.op_bvc(data_ref),
            Inst2Byte::BVSop => self.op_bvs(data_ref),
            Inst2Byte::ALRop => self.op_alr(data_ref),
            Inst2Byte::ANCop => self.op_anc(data_ref),
            Inst2Byte::ARRop => self.op_arr(data_ref),
            Inst2Byte::AXSop => self.op_axs(data_ref),
            Inst2Byte::LAXop => self.op_lax(data_ref),
            Inst2Byte::SAXop => self.op_sax(data_ref),
            Inst2Byte::DCPop => self.op_dcp(data_ref),
            Inst2Byte::ISCop => self.op_isc(data_ref),
            Inst2Byte::RLAop => self.op_rla(data_ref),
            Inst2Byte::RRAop => self.op_rra(data_ref),
            Inst2Byte::SLOop => self.op_slo(data_ref),
            Inst2Byte::SREop => self.op_sre(data_ref),
            Inst2Byte::NOPop => self.op_nop(),
        };
    }

    pub fn execute_inst_3_byte(&mut self, now_inst: Inst3Byte, target_memory: u16, memory: &mut Memory) {
        match now_inst {
            Inst3Byte::LDAop => self.op_lda(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::LDXop => self.op_ldx(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::LDYop => self.op_ldy(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::STAop => self.op_sta(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::STXop => self.op_stx(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::STYop => self.op_sty(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ANDop => self.op_and(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::EORop => self.op_eor(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ORAop => self.op_ora(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::BITop => self.op_bit(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ADCop => self.op_adc(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::SBCop => self.op_sbc(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::CMPop => self.op_cmp(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::CPXop => self.op_cpx(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::CPYop => self.op_cpy(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::INCop => self.op_inc(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::DECop => self.op_dec(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ASLop => self.op_asl(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::LSRop => self.op_lsr(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ROLop => self.op_rol(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::RORop => self.op_ror(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::JMPop => self.op_jmp(target_memory),
            Inst3Byte::JSRop => self.op_jsr(target_memory, memory),
            Inst3Byte::LAXop => self.op_lax(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::SAXop => self.op_sax(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::DCPop => self.op_dcp(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::ISCop => self.op_isc(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::RLAop => self.op_rla(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::RRAop => self.op_rra(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::SLOop => self.op_slo(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::SREop => self.op_sre(memory.get_mut_8bit_value(target_memory)),
            Inst3Byte::SHXop => self.op_shx(target_memory),
            Inst3Byte::SHYop => self.op_shy(target_memory),
            Inst3Byte::NOPop => self.op_nop(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    cycles: u8,
    cycles_pgcr: u8,
    memory_type: MemoryType,
    op_name: CPUInstByte,
}

impl Operation {
    pub fn cycles(&self) -> u8 {
        self.cycles
    }

    pub fn _cycles_pgcr(&self) -> u8 {
        self.cycles_pgcr
    }

    pub fn memory_type(&self) -> MemoryType {
        self.memory_type
    }

    pub fn op_name(&self) -> CPUInstByte {
        self.op_name
    }
}

impl Operation {
    const fn new(cycles: u8, memory_type: MemoryType, op_name: CPUInstByte) -> Operation {
        Operation {
            cycles,
            cycles_pgcr: 0,
            memory_type,
            op_name,
        }
    }

    const fn set_cycles_page_crossed(&mut self, cycles_pgcr: u8) {
        self.cycles_pgcr = cycles_pgcr
    }
}

pub const fn init_all_operations() -> ([Operation; 256], usize) {
    let mut all_operations: [Operation; 256] = [NO_OP; 256];
    let mut oper_counter = 0;

    // Load & Store operations: LDA, LDX, LDY, STA, STX, STY
    
    // LDA operations
    all_operations[0xA9] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::LDAop));
    all_operations[0xA5] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::LDAop));
    all_operations[0xB5] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::LDAop));
    all_operations[0xAD] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::LDAop));
    all_operations[0xBD] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::LDAop));
    all_operations[0xB9] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::LDAop));
    all_operations[0xA1] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::LDAop));
    all_operations[0xB1] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::LDAop));

    all_operations[0xBD].set_cycles_page_crossed(1);
    all_operations[0xB9].set_cycles_page_crossed(1);
    all_operations[0xB1].set_cycles_page_crossed(1);
    oper_counter += 8;

    // LDX operations
    all_operations[0xA2] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::LDXop));
    all_operations[0xA6] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::LDXop));
    all_operations[0xB6] = Operation::new(4, MemoryType::ZeroPageY, CPUInstByte::Two(Inst2Byte::LDXop));
    all_operations[0xAE] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::LDXop));
    all_operations[0xBE] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::LDXop));

    all_operations[0xBE].set_cycles_page_crossed(1);
    oper_counter += 5;

    // LDY operations
    all_operations[0xA0] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::LDYop));
    all_operations[0xA4] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::LDYop));
    all_operations[0xB4] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::LDYop));
    all_operations[0xAC] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::LDYop));
    all_operations[0xBC] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::LDYop));

    all_operations[0xBC].set_cycles_page_crossed(1);
    oper_counter += 5;

    // STA operations
    all_operations[0x85] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::STAop));
    all_operations[0x95] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::STAop));
    all_operations[0x8D] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::STAop));
    all_operations[0x9D] = Operation::new(5, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::STAop));
    all_operations[0x99] = Operation::new(5, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::STAop));
    all_operations[0x81] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::STAop));
    all_operations[0x91] = Operation::new(6, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::STAop));

    oper_counter += 7;

    // STX operations
    all_operations[0x86] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::STXop));
    all_operations[0x96] = Operation::new(4, MemoryType::ZeroPageY, CPUInstByte::Two(Inst2Byte::STXop));
    all_operations[0x8E] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::STXop));

    oper_counter += 3;

    // STY operations
    all_operations[0x84] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::STYop));
    all_operations[0x94] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::STYop));
    all_operations[0x8C] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::STYop));

    oper_counter += 3;

    // Register transfer operations: TAX, TAY, TXA, TYA
    
    // TAX, TAY, TXA, TYA operations
    all_operations[0xAA] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TAXop));
    all_operations[0xA8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TAYop));
    all_operations[0x8A] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TXAop));
    all_operations[0x98] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TYAop));

    oper_counter += 4;

    // Stack operations: TSX, TXS, PHA, PHP, PLA, PLP
    
    // TSX, TXS operations
    all_operations[0xBA] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TSXop));
    all_operations[0x9A] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::TXSop));
    oper_counter += 2;

    // PHA, PHP operations
    all_operations[0x48] = Operation::new(3, MemoryType::Implied, CPUInstByte::One(Inst1Byte::PHAop));
    all_operations[0x08] = Operation::new(3, MemoryType::Implied, CPUInstByte::One(Inst1Byte::PHPop));

    oper_counter += 2;

    // PLA, PLP operations
    all_operations[0x68] = Operation::new(4, MemoryType::Implied, CPUInstByte::One(Inst1Byte::PLAop));
    all_operations[0x28] = Operation::new(4, MemoryType::Implied, CPUInstByte::One(Inst1Byte::PLPop));

    oper_counter += 2;

    // Logical operations: AND, EOR, ORA, BIT

    // AND operations
    all_operations[0x29] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ANDop));
    all_operations[0x25] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ANDop));
    all_operations[0x35] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ANDop));
    all_operations[0x2D] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ANDop));
    all_operations[0x3D] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ANDop));
    all_operations[0x39] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::ANDop));
    all_operations[0x21] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::ANDop));
    all_operations[0x31] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::ANDop));

    all_operations[0x3D].set_cycles_page_crossed(1);
    all_operations[0x39].set_cycles_page_crossed(1);
    all_operations[0x31].set_cycles_page_crossed(1);

    oper_counter += 8;

    // EOR operations
    all_operations[0x49] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::EORop));
    all_operations[0x45] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::EORop));
    all_operations[0x55] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::EORop));
    all_operations[0x4D] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::EORop));
    all_operations[0x5D] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::EORop));
    all_operations[0x59] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::EORop));
    all_operations[0x41] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::EORop));
    all_operations[0x51] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::EORop));

    all_operations[0x5D].set_cycles_page_crossed(1);
    all_operations[0x59].set_cycles_page_crossed(1);
    all_operations[0x51].set_cycles_page_crossed(1);

    oper_counter += 8;

    // ORA operations
    all_operations[0x09] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ORAop));
    all_operations[0x05] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ORAop));
    all_operations[0x15] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ORAop));
    all_operations[0x0D] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ORAop));
    all_operations[0x1D] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ORAop));
    all_operations[0x19] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::ORAop));
    all_operations[0x01] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::ORAop));
    all_operations[0x11] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::ORAop));

    all_operations[0x1D].set_cycles_page_crossed(1);
    all_operations[0x19].set_cycles_page_crossed(1);
    all_operations[0x11].set_cycles_page_crossed(1);

    oper_counter += 8;

    // BIT operations
    all_operations[0x24] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::BITop));
    all_operations[0x2C] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::BITop));

    oper_counter += 2;

    // Arithmetic operations: ADC, SBC, CMP, CPX, CPY
    // ADC operations
    all_operations[0x69] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ADCop));
    all_operations[0x65] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ADCop));
    all_operations[0x75] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ADCop));
    all_operations[0x6D] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ADCop));
    all_operations[0x7D] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ADCop));
    all_operations[0x79] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::ADCop));
    all_operations[0x61] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::ADCop));
    all_operations[0x71] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::ADCop));

    all_operations[0x7D].set_cycles_page_crossed(1);
    all_operations[0x79].set_cycles_page_crossed(1);
    all_operations[0x71].set_cycles_page_crossed(1);

    oper_counter += 8;

    // SBC operations
    all_operations[0xE9] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::SBCop));
    all_operations[0xE5] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::SBCop));
    all_operations[0xF5] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::SBCop));
    all_operations[0xED] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::SBCop));
    all_operations[0xFD] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::SBCop));
    all_operations[0xF9] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::SBCop));
    all_operations[0xE1] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::SBCop));
    all_operations[0xF1] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::SBCop));

    all_operations[0xFD].set_cycles_page_crossed(1);
    all_operations[0xF9].set_cycles_page_crossed(1);
    all_operations[0xF1].set_cycles_page_crossed(1);

    oper_counter += 8;

    // CMP operations
    all_operations[0xC9] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::CMPop));
    all_operations[0xC5] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::CMPop));
    all_operations[0xD5] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::CMPop));
    all_operations[0xCD] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::CMPop));
    all_operations[0xDD] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::CMPop));
    all_operations[0xD9] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::CMPop));
    all_operations[0xC1] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::CMPop));
    all_operations[0xD1] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::CMPop));

    all_operations[0xDD].set_cycles_page_crossed(1);
    all_operations[0xD9].set_cycles_page_crossed(1);
    all_operations[0xD1].set_cycles_page_crossed(1);

    oper_counter += 8;

    // CPX operations
    all_operations[0xE0] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::CPXop));
    all_operations[0xE4] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::CPXop));
    all_operations[0xEC] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::CPXop));

    oper_counter += 3;

    // CPY operations
    all_operations[0xC0] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::CPYop));
    all_operations[0xC4] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::CPYop));
    all_operations[0xCC] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::CPYop));

    oper_counter += 3;

    // Increments and Decrements operations: INC, INX, INY, DEC, DEX, DEY
    // INC operations
    all_operations[0xE6] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::INCop));
    all_operations[0xF6] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::INCop));
    all_operations[0xEE] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::INCop));
    all_operations[0xFE] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::INCop));

    oper_counter += 4;

    // INX, INY operations
    all_operations[0xE8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::INXop));
    all_operations[0xC8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::INYop));

    oper_counter += 2;

    // DEC operations
    all_operations[0xC6] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::DECop));
    all_operations[0xD6] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::DECop));
    all_operations[0xCE] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::DECop));
    all_operations[0xDE] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::DECop));

    oper_counter += 4;

    // DEX, DEY operations
    all_operations[0xCA] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::DEXop));
    all_operations[0x88] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::DEYop));

    oper_counter += 2;

    // Shifts operations: ASL, LSR, ROL, ROR
    // ASL operations
    all_operations[0x0A] = Operation::new(2, MemoryType::Accumulator, CPUInstByte::One(Inst1Byte::ASLop));
    all_operations[0x06] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ASLop));
    all_operations[0x16] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ASLop));
    all_operations[0x0E] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ASLop));
    all_operations[0x1E] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ASLop));

    oper_counter += 5;

    // LSR operations
    all_operations[0x4A] = Operation::new(2, MemoryType::Accumulator, CPUInstByte::One(Inst1Byte::LSRop));
    all_operations[0x46] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::LSRop));
    all_operations[0x56] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::LSRop));
    all_operations[0x4E] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::LSRop));
    all_operations[0x5E] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::LSRop));

    oper_counter += 5;

    // ROL operations
    all_operations[0x2A] = Operation::new(2, MemoryType::Accumulator, CPUInstByte::One(Inst1Byte::ROLop));
    all_operations[0x26] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ROLop));
    all_operations[0x36] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ROLop));
    all_operations[0x2E] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ROLop));
    all_operations[0x3E] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ROLop));

    oper_counter += 5;

    // ROR operations
    all_operations[0x6A] = Operation::new(2, MemoryType::Accumulator, CPUInstByte::One(Inst1Byte::RORop));
    all_operations[0x66] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::RORop));
    all_operations[0x76] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::RORop));
    all_operations[0x6E] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::RORop));
    all_operations[0x7E] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::RORop));

    oper_counter += 5;

    // Jump and Calls operations: JMP, JSR, RTS
    // JMP operations
    all_operations[0x4C] = Operation::new(3, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::JMPop));
    all_operations[0x6C] = Operation::new(5, MemoryType::Indirect, CPUInstByte::Three(Inst3Byte::JMPop));

    oper_counter += 2;

    // JSR and RTS
    all_operations[0x20] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::JSRop));
    all_operations[0x60] = Operation::new(6, MemoryType::Implied, CPUInstByte::One(Inst1Byte::RTSop));

    oper_counter += 2;

    // Branches: BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
    // BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS operations
    all_operations[0x90] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BCCop));
    all_operations[0xB0] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BCSop));
    all_operations[0xF0] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BEQop));
    all_operations[0x30] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BMIop));
    all_operations[0xD0] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BNEop));
    all_operations[0x10] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BPLop));
    all_operations[0x50] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BVCop));
    all_operations[0x70] = Operation::new(2, MemoryType::Relative, CPUInstByte::Two(Inst2Byte::BVSop));

    oper_counter += 8;

    // Status flag changes: CLC, CLD, CLI, CLV, SEC, SED, SEI
    // CLC, CLD, CLI, CLV, SEC, SED, SEI operations
    all_operations[0x18] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::CLCop));
    all_operations[0xD8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::CLDop));
    all_operations[0x58] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::CLIop));
    all_operations[0xB8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::CLVop));
    all_operations[0x38] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::SECop));
    all_operations[0xF8] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::SEDop));
    all_operations[0x78] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::SEIop));

    oper_counter += 7;

    // System functions: BRK, NOP, RTI
    // BRK, NOP, RTI operations
    all_operations[0x00] = Operation::new(7, MemoryType::Implied, CPUInstByte::One(Inst1Byte::BRKop));
    all_operations[0xEA] = Operation::new(2, MemoryType::Implied, CPUInstByte::One(Inst1Byte::NOPop));
    all_operations[0x40] = Operation::new(6, MemoryType::Implied, CPUInstByte::One(Inst1Byte::RTIop));

    oper_counter += 3;

    // UNOFFICIAL
    
    // Combined operations
    // ALR(ASR), ANC(AAC), ARR, AXS(SBX,SAX), LAX, SAX(AAX, AXS) operations
    
    // ALR, ANC, ARR, AXS operations
    all_operations[0x4B] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ALRop));
    all_operations[0x0B] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ANCop));
    all_operations[0x6B] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::ARRop));
    all_operations[0xCB] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::AXSop));

    oper_counter += 4;

    // LAX operations
    all_operations[0xA7] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::LAXop));
    all_operations[0xB7] = Operation::new(4, MemoryType::ZeroPageY, CPUInstByte::Two(Inst2Byte::LAXop));
    all_operations[0xAF] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::LAXop));
    all_operations[0xBF] = Operation::new(4, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::LAXop));
    all_operations[0xA3] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::LAXop));
    all_operations[0xB3] = Operation::new(5, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::LAXop));

    oper_counter += 6;

    // SAX operations
    all_operations[0x87] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::SAXop));
    all_operations[0x97] = Operation::new(4, MemoryType::ZeroPageY, CPUInstByte::Two(Inst2Byte::SAXop));
    all_operations[0x83] = Operation::new(6, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::SAXop));
    all_operations[0x8F] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::SAXop));

    oper_counter += 4;

    // RMW instructions
    // DCP(DCM), ISC(ISB,INS), RLA, RRA, SLO(ASO), SRE(LSE) operations

    // DCP(DCM) operations
    all_operations[0xC7] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::DCPop));
    all_operations[0xD7] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::DCPop));
    all_operations[0xCF] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::DCPop));
    all_operations[0xDF] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::DCPop));
    all_operations[0xDB] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::DCPop));
    all_operations[0xC3] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::DCPop));
    all_operations[0xD3] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::DCPop));

    oper_counter += 7;

    // ISC(ISB,INS) operations
    all_operations[0xE7] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::ISCop));
    all_operations[0xF7] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::ISCop));
    all_operations[0xEF] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::ISCop));
    all_operations[0xFF] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::ISCop));
    all_operations[0xFB] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::ISCop));
    all_operations[0xE3] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::ISCop));
    all_operations[0xF3] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::ISCop));

    oper_counter += 7;

    // RLA operations
    all_operations[0x27] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::RLAop));
    all_operations[0x37] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::RLAop));
    all_operations[0x2F] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::RLAop));
    all_operations[0x3F] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::RLAop));
    all_operations[0x3B] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::RLAop));
    all_operations[0x23] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::RLAop));
    all_operations[0x33] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::RLAop));

    oper_counter += 7;

    // RRA operations
    all_operations[0x67] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::RRAop));
    all_operations[0x77] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::RRAop));
    all_operations[0x6F] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::RRAop));
    all_operations[0x7F] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::RRAop));
    all_operations[0x7B] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::RRAop));
    all_operations[0x63] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::RRAop));
    all_operations[0x73] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::RRAop));

    oper_counter += 7;

    // SLO(ASO) operations
    all_operations[0x07] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::SLOop));
    all_operations[0x17] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::SLOop));
    all_operations[0x0F] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::SLOop));
    all_operations[0x1F] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::SLOop));
    all_operations[0x1B] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::SLOop));
    all_operations[0x03] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::SLOop));
    all_operations[0x13] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::SLOop));

    oper_counter += 7;

    // SRE(LSE) operations
    all_operations[0x47] = Operation::new(5, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::SREop));
    all_operations[0x57] = Operation::new(6, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::SREop));
    all_operations[0x4F] = Operation::new(6, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::SREop));
    all_operations[0x5F] = Operation::new(7, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::SREop));
    all_operations[0x5B] = Operation::new(7, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::SREop));
    all_operations[0x43] = Operation::new(8, MemoryType::IndirectX, CPUInstByte::Two(Inst2Byte::SREop));
    all_operations[0x53] = Operation::new(8, MemoryType::IndirectY, CPUInstByte::Two(Inst2Byte::SREop));

    oper_counter += 7;

    // Dublicates SBC operation
    // SBC operation
    all_operations[0xEB] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::SBCop));
    oper_counter += 1;

    // Incorrect memory modes SHX(SXA, XAS), SHY(SYA, SAY) operations
    // SHX(SXA, XAS) and SHY(SYA, SAY) operations
    all_operations[0x9E] = Operation::new(5, MemoryType::AbsoluteY, CPUInstByte::Three(Inst3Byte::SHXop));
    all_operations[0x9C] = Operation::new(5, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::SHYop));
    oper_counter += 2;

    // NOPs opeartions
    // NOP Implied operatoins
    all_operations[0x1A] = all_operations[0xEA];
    all_operations[0x3A] = all_operations[0xEA];
    all_operations[0x5A] = all_operations[0xEA];
    all_operations[0x7A] = all_operations[0xEA];
    all_operations[0xDA] = all_operations[0xEA];
    all_operations[0xFA] = all_operations[0xEA];
    oper_counter += 6;

    // NOP Immediate opeartions
    all_operations[0x80] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0x82] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0x89] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0xC2] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0xE2] = Operation::new(2, MemoryType::Immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    oper_counter += 5;

    // NOP Absolute
    all_operations[0x0C] = Operation::new(4, MemoryType::Absolute, CPUInstByte::Three(Inst3Byte::NOPop));
    all_operations[0x1C] = Operation::new(4, MemoryType::AbsoluteX, CPUInstByte::Three(Inst3Byte::NOPop));
    all_operations[0x3C] = all_operations[0x1C];
    all_operations[0x5C] = all_operations[0x1C];
    all_operations[0x7C] = all_operations[0x1C];
    all_operations[0xDC] = all_operations[0x1C];
    all_operations[0xFC] = all_operations[0x1C];
    oper_counter += 7;

    // NOP ZeroPage
    all_operations[0x04] = Operation::new(3, MemoryType::ZeroPage, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0x44] = all_operations[0x04];
    all_operations[0x64] = all_operations[0x04];
    oper_counter += 3;

    // NOP ZeroPageX
    all_operations[0x14] = Operation::new(4, MemoryType::ZeroPageX, CPUInstByte::Two(Inst2Byte::NOPop));
    all_operations[0x34] = all_operations[0x14];
    all_operations[0x54] = all_operations[0x14];
    all_operations[0x74] = all_operations[0x14];
    all_operations[0xD4] = all_operations[0x14];
    all_operations[0xF4] = all_operations[0x14];
    oper_counter += 6;

    // STP operations
    all_operations[0x02] = Operation::new(0, MemoryType::Implied, CPUInstByte::One(Inst1Byte::STPop));
    all_operations[0x12] = all_operations[0x02];
    all_operations[0x22] = all_operations[0x02];
    all_operations[0x32] = all_operations[0x02];
    all_operations[0x42] = all_operations[0x02];
    all_operations[0x52] = all_operations[0x02];
    all_operations[0x62] = all_operations[0x02];
    all_operations[0x72] = all_operations[0x02];
    all_operations[0x92] = all_operations[0x02];
    all_operations[0xB2] = all_operations[0x02];
    all_operations[0xD2] = all_operations[0x02];
    all_operations[0xF2] = all_operations[0x02];

    oper_counter += 12;

    (all_operations, oper_counter)
}

impl std::fmt::Display for CPUInstByte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CPUInstByte::One(inst) => write!(f, "{inst:?}"),
            CPUInstByte::Two(inst) => write!(f, "{inst:?}"),
            CPUInstByte::Three(inst) => write!(f, "{inst:?}"),
            CPUInstByte::NoOp => write!(f, "NoOp"),
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}", self.op_name, self.memory_type)
    }
}
