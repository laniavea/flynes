use crate::bus::Bus;
use crate::cpu::Cpu;
use crate::memory::MemoryType;

mod arithmetic;
mod branches;
mod increment_decrement;
mod jumps_calls;
mod load_store;
mod logical;
mod register_transfer;
mod shifts;
mod stack_operations;
mod status_flag_changes;
mod system_functions;
mod unofficial_combined;
mod unofficial_other;
mod unofficial_rmw;

pub mod shared_ops;

const NO_OP: Operation = Operation {
    cycles: 0,
    cycles_pgcr: 0,
    code: 0,
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
    XAAop,
    AHXop,
    LAX2op,
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
    AHXop,
    TASop,
    LASop,
}

impl Cpu {
    pub fn execute_inst_1_byte(&mut self, now_inst: Inst1Byte, bus: &mut Bus) {
        match now_inst {
            Inst1Byte::TAXop => self.op_tax(),
            Inst1Byte::TAYop => self.op_tay(),
            Inst1Byte::TXAop => self.op_txa(),
            Inst1Byte::TYAop => self.op_tya(),
            Inst1Byte::TSXop => self.op_tsx(),
            Inst1Byte::TXSop => self.op_txs(),
            Inst1Byte::PHAop => self.op_pha(bus),
            Inst1Byte::PHPop => self.op_php(bus),
            Inst1Byte::PLAop => self.op_pla(bus),
            Inst1Byte::PLPop => self.op_plp(bus),
            Inst1Byte::INXop => self.op_inx(),
            Inst1Byte::INYop => self.op_iny(),
            Inst1Byte::DEXop => self.op_dex(),
            Inst1Byte::DEYop => self.op_dey(),
            Inst1Byte::ASLop => self.op_asl_acc(),
            Inst1Byte::LSRop => self.op_lsr_acc(),
            Inst1Byte::ROLop => self.op_rol_acc(),
            Inst1Byte::RORop => self.op_ror_acc(),
            Inst1Byte::RTSop => self.op_rts(bus),
            Inst1Byte::CLCop => self.op_clc(),
            Inst1Byte::CLDop => self.op_cld(),
            Inst1Byte::CLIop => self.op_cli(),
            Inst1Byte::CLVop => self.op_clv(),
            Inst1Byte::SECop => self.op_sec(),
            Inst1Byte::SEDop => self.op_sed(),
            Inst1Byte::SEIop => self.op_sei(),
            Inst1Byte::BRKop => self.op_brk(bus),
            Inst1Byte::NOPop => self.op_nop(),
            Inst1Byte::RTIop => self.op_rti(bus),
            Inst1Byte::STPop => self.op_stp(),
        }
    }

    pub fn execute_inst_2_byte(&mut self, bus: &mut Bus, now_inst: Inst2Byte, conv_data_ref: u16) {
        match now_inst {
            Inst2Byte::LDAop => self.op_lda(bus, conv_data_ref),
            Inst2Byte::LDXop => self.op_ldx(bus, conv_data_ref),
            Inst2Byte::LDYop => self.op_ldy(bus, conv_data_ref),
            Inst2Byte::STAop => self.op_sta(bus, conv_data_ref),
            Inst2Byte::STXop => self.op_stx(bus, conv_data_ref),
            Inst2Byte::STYop => self.op_sty(bus, conv_data_ref),
            Inst2Byte::ANDop => self.op_and(bus, conv_data_ref),
            Inst2Byte::EORop => self.op_eor(bus, conv_data_ref),
            Inst2Byte::ORAop => self.op_ora(bus, conv_data_ref),
            Inst2Byte::BITop => self.op_bit(bus, conv_data_ref),
            Inst2Byte::ADCop => self.op_adc(bus, conv_data_ref),
            Inst2Byte::SBCop => self.op_sbc(bus, conv_data_ref),
            Inst2Byte::CMPop => self.op_cmp(bus, conv_data_ref),
            Inst2Byte::CPXop => self.op_cpx(bus, conv_data_ref),
            Inst2Byte::CPYop => self.op_cpy(bus, conv_data_ref),
            Inst2Byte::INCop => self.op_inc(bus, conv_data_ref),
            Inst2Byte::DECop => self.op_dec(bus, conv_data_ref),
            Inst2Byte::ASLop => self.op_asl(bus, conv_data_ref),
            Inst2Byte::LSRop => self.op_lsr(bus, conv_data_ref),
            Inst2Byte::ROLop => self.op_rol(bus, conv_data_ref),
            Inst2Byte::RORop => self.op_ror(bus, conv_data_ref),
            Inst2Byte::BCCop => self.op_bcc(bus, conv_data_ref),
            Inst2Byte::BCSop => self.op_bcs(bus, conv_data_ref),
            Inst2Byte::BEQop => self.op_beq(bus, conv_data_ref),
            Inst2Byte::BMIop => self.op_bmi(bus, conv_data_ref),
            Inst2Byte::BNEop => self.op_bne(bus, conv_data_ref),
            Inst2Byte::BPLop => self.op_bpl(bus, conv_data_ref),
            Inst2Byte::BVCop => self.op_bvc(bus, conv_data_ref),
            Inst2Byte::BVSop => self.op_bvs(bus, conv_data_ref),
            Inst2Byte::ALRop => self.op_alr(bus, conv_data_ref),
            Inst2Byte::ANCop => self.op_anc(bus, conv_data_ref),
            Inst2Byte::ARRop => self.op_arr(bus, conv_data_ref),
            Inst2Byte::AXSop => self.op_axs(bus, conv_data_ref),
            Inst2Byte::LAXop => self.op_lax(bus, conv_data_ref),
            Inst2Byte::SAXop => self.op_sax(bus, conv_data_ref),
            Inst2Byte::DCPop => self.op_dcp(bus, conv_data_ref),
            Inst2Byte::ISCop => self.op_isc(bus, conv_data_ref),
            Inst2Byte::RLAop => self.op_rla(bus, conv_data_ref),
            Inst2Byte::RRAop => self.op_rra(bus, conv_data_ref),
            Inst2Byte::SLOop => self.op_slo(bus, conv_data_ref),
            Inst2Byte::SREop => self.op_sre(bus, conv_data_ref),
            Inst2Byte::NOPop => self.op_nop(),
            Inst2Byte::XAAop => self.op_xaa(bus, conv_data_ref),
            Inst2Byte::AHXop => self.op_ahx(bus, conv_data_ref),
            Inst2Byte::LAX2op => self.op_lax_other_ver(bus, conv_data_ref),
        };
    }

    pub fn execute_inst_3_byte(&mut self, bus: &mut Bus, now_inst: Inst3Byte, conv_data_ref: u16) {
        match now_inst {
            Inst3Byte::LDAop => self.op_lda(bus, conv_data_ref),
            Inst3Byte::LDXop => self.op_ldx(bus, conv_data_ref),
            Inst3Byte::LDYop => self.op_ldy(bus, conv_data_ref),
            Inst3Byte::STAop => self.op_sta(bus, conv_data_ref),
            Inst3Byte::STXop => self.op_stx(bus, conv_data_ref),
            Inst3Byte::STYop => self.op_sty(bus, conv_data_ref),
            Inst3Byte::ANDop => self.op_and(bus, conv_data_ref),
            Inst3Byte::EORop => self.op_eor(bus, conv_data_ref),
            Inst3Byte::ORAop => self.op_ora(bus, conv_data_ref),
            Inst3Byte::BITop => self.op_bit(bus, conv_data_ref),
            Inst3Byte::ADCop => self.op_adc(bus, conv_data_ref),
            Inst3Byte::SBCop => self.op_sbc(bus, conv_data_ref),
            Inst3Byte::CMPop => self.op_cmp(bus, conv_data_ref),
            Inst3Byte::CPXop => self.op_cpx(bus, conv_data_ref),
            Inst3Byte::CPYop => self.op_cpy(bus, conv_data_ref),
            Inst3Byte::INCop => self.op_inc(bus, conv_data_ref),
            Inst3Byte::DECop => self.op_dec(bus, conv_data_ref),
            Inst3Byte::ASLop => self.op_asl(bus, conv_data_ref),
            Inst3Byte::LSRop => self.op_lsr(bus, conv_data_ref),
            Inst3Byte::ROLop => self.op_rol(bus, conv_data_ref),
            Inst3Byte::RORop => self.op_ror(bus, conv_data_ref),
            Inst3Byte::JMPop => self.op_jmp(conv_data_ref),
            Inst3Byte::JSRop => self.op_jsr(bus, conv_data_ref),
            Inst3Byte::LAXop => self.op_lax(bus, conv_data_ref),
            Inst3Byte::SAXop => self.op_sax(bus, conv_data_ref),
            Inst3Byte::DCPop => self.op_dcp(bus, conv_data_ref),
            Inst3Byte::ISCop => self.op_isc(bus, conv_data_ref),
            Inst3Byte::RLAop => self.op_rla(bus, conv_data_ref),
            Inst3Byte::RRAop => self.op_rra(bus, conv_data_ref),
            Inst3Byte::SLOop => self.op_slo(bus, conv_data_ref),
            Inst3Byte::SREop => self.op_sre(bus, conv_data_ref),
            Inst3Byte::SHXop => self.op_shx(bus, conv_data_ref),
            Inst3Byte::SHYop => self.op_shy(bus, conv_data_ref),
            Inst3Byte::NOPop => self.op_nop(),
            Inst3Byte::AHXop => self.op_ahx(bus, conv_data_ref),
            Inst3Byte::TASop => self.op_tas(bus, conv_data_ref),
            Inst3Byte::LASop => self.op_las(bus, conv_data_ref),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Operation {
    cycles: u8,
    cycles_pgcr: u8,
    code: u8,
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

    pub fn code(&self) -> u8 {
        self.code
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
            code: 0,
            memory_type,
            op_name,
        }
    }

    const fn set_cycles_page_crossed(&mut self, cycles_pgcr: u8) {
        self.cycles_pgcr = cycles_pgcr
    }
}

pub const fn init_cpu_operations() -> ([Operation; 256], usize) {
    let mut ops: [Operation; 256] = [NO_OP; 256];
    let mut oper_counter = 0;

    let accumulator = MemoryType::Accumulator;
    let immediate = MemoryType::Immediate;
    let implied = MemoryType::Implied;
    let relative = MemoryType::Relative;
    let zero_page = MemoryType::ZeroPage;
    let zero_page_x = MemoryType::ZeroPageX;
    let zero_page_y = MemoryType::ZeroPageY;
    let absolute = MemoryType::Absolute;
    let absolute_x = MemoryType::AbsoluteX;
    let absolute_y = MemoryType::AbsoluteY;
    let indirect = MemoryType::Indirect;
    let indirect_x = MemoryType::IndirectX;
    let indirect_y = MemoryType::IndirectY;

    // Load & Store operations: LDA, LDX, LDY, STA, STX, STY

    // LDA operations
    ops[0xA9] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::LDAop));
    ops[0xA5] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::LDAop));
    ops[0xB5] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::LDAop));
    ops[0xAD] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::LDAop));
    ops[0xBD] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::LDAop));
    ops[0xB9] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::LDAop));
    ops[0xA1] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::LDAop));
    ops[0xB1] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::LDAop));

    ops[0xBD].set_cycles_page_crossed(1);
    ops[0xB9].set_cycles_page_crossed(1);
    ops[0xB1].set_cycles_page_crossed(1);
    oper_counter += 8;

    // LDX operations
    ops[0xA2] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::LDXop));
    ops[0xA6] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::LDXop));
    ops[0xB6] = Operation::new(4, zero_page_y, CPUInstByte::Two(Inst2Byte::LDXop));
    ops[0xAE] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::LDXop));
    ops[0xBE] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::LDXop));

    ops[0xBE].set_cycles_page_crossed(1);
    oper_counter += 5;

    // LDY operations
    ops[0xA0] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::LDYop));
    ops[0xA4] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::LDYop));
    ops[0xB4] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::LDYop));
    ops[0xAC] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::LDYop));
    ops[0xBC] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::LDYop));

    ops[0xBC].set_cycles_page_crossed(1);
    oper_counter += 5;

    // STA operations
    ops[0x85] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::STAop));
    ops[0x95] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::STAop));
    ops[0x8D] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::STAop));
    ops[0x9D] = Operation::new(5, absolute_x, CPUInstByte::Three(Inst3Byte::STAop));
    ops[0x99] = Operation::new(5, absolute_y, CPUInstByte::Three(Inst3Byte::STAop));
    ops[0x81] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::STAop));
    ops[0x91] = Operation::new(6, indirect_y, CPUInstByte::Two(Inst2Byte::STAop));

    oper_counter += 7;

    // STX operations
    ops[0x86] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::STXop));
    ops[0x96] = Operation::new(4, zero_page_y, CPUInstByte::Two(Inst2Byte::STXop));
    ops[0x8E] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::STXop));

    oper_counter += 3;

    // STY operations
    ops[0x84] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::STYop));
    ops[0x94] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::STYop));
    ops[0x8C] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::STYop));

    oper_counter += 3;

    // Register transfer operations: TAX, TAY, TXA, TYA

    // TAX, TAY, TXA, TYA operations
    ops[0xAA] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TAXop));
    ops[0xA8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TAYop));
    ops[0x8A] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TXAop));
    ops[0x98] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TYAop));

    oper_counter += 4;

    // Stack operations: TSX, TXS, PHA, PHP, PLA, PLP

    // TSX, TXS operations
    ops[0xBA] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TSXop));
    ops[0x9A] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::TXSop));
    oper_counter += 2;

    // PHA, PHP operations
    ops[0x48] = Operation::new(3, implied, CPUInstByte::One(Inst1Byte::PHAop));
    ops[0x08] = Operation::new(3, implied, CPUInstByte::One(Inst1Byte::PHPop));

    oper_counter += 2;

    // PLA, PLP operations
    ops[0x68] = Operation::new(4, implied, CPUInstByte::One(Inst1Byte::PLAop));
    ops[0x28] = Operation::new(4, implied, CPUInstByte::One(Inst1Byte::PLPop));

    oper_counter += 2;

    // Logical operations: AND, EOR, ORA, BIT

    // AND operations
    ops[0x29] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ANDop));
    ops[0x25] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::ANDop));
    ops[0x35] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::ANDop));
    ops[0x2D] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::ANDop));
    ops[0x3D] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::ANDop));
    ops[0x39] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::ANDop));
    ops[0x21] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::ANDop));
    ops[0x31] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::ANDop));

    ops[0x3D].set_cycles_page_crossed(1);
    ops[0x39].set_cycles_page_crossed(1);
    ops[0x31].set_cycles_page_crossed(1);

    oper_counter += 8;

    // EOR operations
    ops[0x49] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::EORop));
    ops[0x45] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::EORop));
    ops[0x55] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::EORop));
    ops[0x4D] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::EORop));
    ops[0x5D] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::EORop));
    ops[0x59] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::EORop));
    ops[0x41] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::EORop));
    ops[0x51] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::EORop));

    ops[0x5D].set_cycles_page_crossed(1);
    ops[0x59].set_cycles_page_crossed(1);
    ops[0x51].set_cycles_page_crossed(1);

    oper_counter += 8;

    // ORA operations
    ops[0x09] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ORAop));
    ops[0x05] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::ORAop));
    ops[0x15] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::ORAop));
    ops[0x0D] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::ORAop));
    ops[0x1D] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::ORAop));
    ops[0x19] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::ORAop));
    ops[0x01] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::ORAop));
    ops[0x11] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::ORAop));

    ops[0x1D].set_cycles_page_crossed(1);
    ops[0x19].set_cycles_page_crossed(1);
    ops[0x11].set_cycles_page_crossed(1);

    oper_counter += 8;

    // BIT operations
    ops[0x24] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::BITop));
    ops[0x2C] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::BITop));

    oper_counter += 2;

    // Arithmetic operations: ADC, SBC, CMP, CPX, CPY
    // ADC operations
    ops[0x69] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ADCop));
    ops[0x65] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::ADCop));
    ops[0x75] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::ADCop));
    ops[0x6D] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::ADCop));
    ops[0x7D] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::ADCop));
    ops[0x79] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::ADCop));
    ops[0x61] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::ADCop));
    ops[0x71] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::ADCop));

    ops[0x7D].set_cycles_page_crossed(1);
    ops[0x79].set_cycles_page_crossed(1);
    ops[0x71].set_cycles_page_crossed(1);

    oper_counter += 8;

    // SBC operations
    ops[0xE9] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::SBCop));
    ops[0xE5] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::SBCop));
    ops[0xF5] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::SBCop));
    ops[0xED] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::SBCop));
    ops[0xFD] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::SBCop));
    ops[0xF9] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::SBCop));
    ops[0xE1] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::SBCop));
    ops[0xF1] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::SBCop));

    ops[0xFD].set_cycles_page_crossed(1);
    ops[0xF9].set_cycles_page_crossed(1);
    ops[0xF1].set_cycles_page_crossed(1);

    oper_counter += 8;

    // CMP operations
    ops[0xC9] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::CMPop));
    ops[0xC5] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::CMPop));
    ops[0xD5] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::CMPop));
    ops[0xCD] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::CMPop));
    ops[0xDD] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::CMPop));
    ops[0xD9] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::CMPop));
    ops[0xC1] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::CMPop));
    ops[0xD1] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::CMPop));

    ops[0xDD].set_cycles_page_crossed(1);
    ops[0xD9].set_cycles_page_crossed(1);
    ops[0xD1].set_cycles_page_crossed(1);

    oper_counter += 8;

    // CPX operations
    ops[0xE0] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::CPXop));
    ops[0xE4] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::CPXop));
    ops[0xEC] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::CPXop));

    oper_counter += 3;

    // CPY operations
    ops[0xC0] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::CPYop));
    ops[0xC4] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::CPYop));
    ops[0xCC] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::CPYop));

    oper_counter += 3;

    // Increments and Decrements operations: INC, INX, INY, DEC, DEX, DEY
    // INC operations
    ops[0xE6] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::INCop));
    ops[0xF6] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::INCop));
    ops[0xEE] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::INCop));
    ops[0xFE] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::INCop));

    oper_counter += 4;

    // INX, INY operations
    ops[0xE8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::INXop));
    ops[0xC8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::INYop));

    oper_counter += 2;

    // DEC operations
    ops[0xC6] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::DECop));
    ops[0xD6] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::DECop));
    ops[0xCE] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::DECop));
    ops[0xDE] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::DECop));

    oper_counter += 4;

    // DEX, DEY operations
    ops[0xCA] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::DEXop));
    ops[0x88] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::DEYop));

    oper_counter += 2;

    // Shifts operations: ASL, LSR, ROL, ROR
    // ASL operations
    ops[0x0A] = Operation::new(2, accumulator, CPUInstByte::One(Inst1Byte::ASLop));
    ops[0x06] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::ASLop));
    ops[0x16] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::ASLop));
    ops[0x0E] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::ASLop));
    ops[0x1E] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::ASLop));

    oper_counter += 5;

    // LSR operations
    ops[0x4A] = Operation::new(2, accumulator, CPUInstByte::One(Inst1Byte::LSRop));
    ops[0x46] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::LSRop));
    ops[0x56] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::LSRop));
    ops[0x4E] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::LSRop));
    ops[0x5E] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::LSRop));

    oper_counter += 5;

    // ROL operations
    ops[0x2A] = Operation::new(2, accumulator, CPUInstByte::One(Inst1Byte::ROLop));
    ops[0x26] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::ROLop));
    ops[0x36] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::ROLop));
    ops[0x2E] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::ROLop));
    ops[0x3E] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::ROLop));

    oper_counter += 5;

    // ROR operations
    ops[0x6A] = Operation::new(2, accumulator, CPUInstByte::One(Inst1Byte::RORop));
    ops[0x66] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::RORop));
    ops[0x76] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::RORop));
    ops[0x6E] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::RORop));
    ops[0x7E] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::RORop));

    oper_counter += 5;

    // Jump and Calls operations: JMP, JSR, RTS
    // JMP operations
    ops[0x4C] = Operation::new(3, absolute, CPUInstByte::Three(Inst3Byte::JMPop));
    ops[0x6C] = Operation::new(5, indirect, CPUInstByte::Three(Inst3Byte::JMPop));

    oper_counter += 2;

    // JSR and RTS
    ops[0x20] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::JSRop));
    ops[0x60] = Operation::new(6, implied, CPUInstByte::One(Inst1Byte::RTSop));

    oper_counter += 2;

    // Branches: BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS
    // BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS operations
    ops[0x90] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BCCop));
    ops[0xB0] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BCSop));
    ops[0xF0] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BEQop));
    ops[0x30] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BMIop));
    ops[0xD0] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BNEop));
    ops[0x10] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BPLop));
    ops[0x50] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BVCop));
    ops[0x70] = Operation::new(2, relative, CPUInstByte::Two(Inst2Byte::BVSop));

    oper_counter += 8;

    // Status flag changes: CLC, CLD, CLI, CLV, SEC, SED, SEI
    // CLC, CLD, CLI, CLV, SEC, SED, SEI operations
    ops[0x18] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::CLCop));
    ops[0xD8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::CLDop));
    ops[0x58] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::CLIop));
    ops[0xB8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::CLVop));
    ops[0x38] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::SECop));
    ops[0xF8] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::SEDop));
    ops[0x78] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::SEIop));

    oper_counter += 7;

    // System functions: BRK, NOP, RTI
    // BRK, NOP, RTI operations
    ops[0x00] = Operation::new(7, implied, CPUInstByte::One(Inst1Byte::BRKop));
    ops[0xEA] = Operation::new(2, implied, CPUInstByte::One(Inst1Byte::NOPop));
    ops[0x40] = Operation::new(6, implied, CPUInstByte::One(Inst1Byte::RTIop));

    oper_counter += 3;

    // UNOFFICIAL

    // Combined operations
    // ALR(ASR), ANC(AAC), ARR, AXS(SBX,SAX), LAX, SAX(AAX, AXS) operations

    // ALR, ANC, ARR, AXS operations
    ops[0x4B] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ALRop));
    ops[0x0B] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ANCop));
    ops[0x2B] = ops[0x0B];
    ops[0x6B] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::ARRop));
    ops[0xCB] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::AXSop));

    oper_counter += 5;

    // LAX operations
    ops[0xA7] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::LAXop));
    ops[0xB7] = Operation::new(4, zero_page_y, CPUInstByte::Two(Inst2Byte::LAXop));
    ops[0xAF] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::LAXop));
    ops[0xBF] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::LAXop));
    ops[0xA3] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::LAXop));
    ops[0xB3] = Operation::new(5, indirect_y, CPUInstByte::Two(Inst2Byte::LAXop));

    oper_counter += 6;

    // SAX operations
    ops[0x87] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::SAXop));
    ops[0x97] = Operation::new(4, zero_page_y, CPUInstByte::Two(Inst2Byte::SAXop));
    ops[0x83] = Operation::new(6, indirect_x, CPUInstByte::Two(Inst2Byte::SAXop));
    ops[0x8F] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::SAXop));

    oper_counter += 4;

    // RMW instructions
    // DCP(DCM), ISC(ISB,INS), RLA, RRA, SLO(ASO), SRE(LSE) operations

    // DCP(DCM) operations
    ops[0xC7] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::DCPop));
    ops[0xD7] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::DCPop));
    ops[0xCF] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::DCPop));
    ops[0xDF] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::DCPop));
    ops[0xDB] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::DCPop));
    ops[0xC3] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::DCPop));
    ops[0xD3] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::DCPop));

    oper_counter += 7;

    // ISC(ISB,INS) operations
    ops[0xE7] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::ISCop));
    ops[0xF7] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::ISCop));
    ops[0xEF] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::ISCop));
    ops[0xFF] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::ISCop));
    ops[0xFB] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::ISCop));
    ops[0xE3] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::ISCop));
    ops[0xF3] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::ISCop));

    oper_counter += 7;

    // RLA operations
    ops[0x27] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::RLAop));
    ops[0x37] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::RLAop));
    ops[0x2F] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::RLAop));
    ops[0x3F] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::RLAop));
    ops[0x3B] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::RLAop));
    ops[0x23] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::RLAop));
    ops[0x33] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::RLAop));

    oper_counter += 7;

    // RRA operations
    ops[0x67] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::RRAop));
    ops[0x77] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::RRAop));
    ops[0x6F] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::RRAop));
    ops[0x7F] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::RRAop));
    ops[0x7B] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::RRAop));
    ops[0x63] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::RRAop));
    ops[0x73] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::RRAop));

    oper_counter += 7;

    // SLO(ASO) operations
    ops[0x07] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::SLOop));
    ops[0x17] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::SLOop));
    ops[0x0F] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::SLOop));
    ops[0x1F] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::SLOop));
    ops[0x1B] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::SLOop));
    ops[0x03] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::SLOop));
    ops[0x13] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::SLOop));

    oper_counter += 7;

    // SRE(LSE) operations
    ops[0x47] = Operation::new(5, zero_page, CPUInstByte::Two(Inst2Byte::SREop));
    ops[0x57] = Operation::new(6, zero_page_x, CPUInstByte::Two(Inst2Byte::SREop));
    ops[0x4F] = Operation::new(6, absolute, CPUInstByte::Three(Inst3Byte::SREop));
    ops[0x5F] = Operation::new(7, absolute_x, CPUInstByte::Three(Inst3Byte::SREop));
    ops[0x5B] = Operation::new(7, absolute_y, CPUInstByte::Three(Inst3Byte::SREop));
    ops[0x43] = Operation::new(8, indirect_x, CPUInstByte::Two(Inst2Byte::SREop));
    ops[0x53] = Operation::new(8, indirect_y, CPUInstByte::Two(Inst2Byte::SREop));

    oper_counter += 7;

    // Dublicates SBC operation
    // SBC operation
    ops[0xEB] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::SBCop));
    oper_counter += 1;

    // Incorrect memory modes SHX(SXA, XAS), SHY(SYA, SAY) operations
    // SHX(SXA, XAS) and SHY(SYA, SAY) operations
    ops[0x9E] = Operation::new(5, absolute_y, CPUInstByte::Three(Inst3Byte::SHXop));
    ops[0x9C] = Operation::new(5, absolute_x, CPUInstByte::Three(Inst3Byte::SHYop));
    oper_counter += 2;

    // NOPs opeartions
    // NOP Implied operatoins
    ops[0x1A] = ops[0xEA];
    ops[0x3A] = ops[0xEA];
    ops[0x5A] = ops[0xEA];
    ops[0x7A] = ops[0xEA];
    ops[0xDA] = ops[0xEA];
    ops[0xFA] = ops[0xEA];
    oper_counter += 6;

    // NOP Immediate opeartions
    ops[0x80] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0x82] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0x89] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0xC2] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0xE2] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::NOPop));
    oper_counter += 5;

    // NOP Absolute
    ops[0x0C] = Operation::new(4, absolute, CPUInstByte::Three(Inst3Byte::NOPop));
    ops[0x1C] = Operation::new(4, absolute_x, CPUInstByte::Three(Inst3Byte::NOPop));
    ops[0x3C] = ops[0x1C];
    ops[0x5C] = ops[0x1C];
    ops[0x7C] = ops[0x1C];
    ops[0xDC] = ops[0x1C];
    ops[0xFC] = ops[0x1C];
    oper_counter += 7;

    // NOP ZeroPage
    ops[0x04] = Operation::new(3, zero_page, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0x44] = ops[0x04];
    ops[0x64] = ops[0x04];
    oper_counter += 3;

    // NOP ZeroPageX
    ops[0x14] = Operation::new(4, zero_page_x, CPUInstByte::Two(Inst2Byte::NOPop));
    ops[0x34] = ops[0x14];
    ops[0x54] = ops[0x14];
    ops[0x74] = ops[0x14];
    ops[0xD4] = ops[0x14];
    ops[0xF4] = ops[0x14];
    oper_counter += 6;

    // STP (KIL, JAM, HLT) operations
    ops[0x02] = Operation::new(0, implied, CPUInstByte::One(Inst1Byte::STPop));
    ops[0x12] = ops[0x02];
    ops[0x22] = ops[0x02];
    ops[0x32] = ops[0x02];
    ops[0x42] = ops[0x02];
    ops[0x52] = ops[0x02];
    ops[0x62] = ops[0x02];
    ops[0x72] = ops[0x02];
    ops[0x92] = ops[0x02];
    ops[0xB2] = ops[0x02];
    ops[0xD2] = ops[0x02];
    ops[0xF2] = ops[0x02];

    oper_counter += 12;

    // XAA (ANE) operations
    ops[0x8B] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::XAAop));
    oper_counter += 1;

    // AHX (AXA, SHA) operations
    ops[0x93] = Operation::new(6, indirect_y, CPUInstByte::Two(Inst2Byte::AHXop));
    ops[0x9F] = Operation::new(5, absolute_y, CPUInstByte::Three(Inst3Byte::AHXop));
    oper_counter += 2;

    // TAS (XAS, SHS) operations
    ops[0x9B] = Operation::new(5, absolute_y, CPUInstByte::Three(Inst3Byte::TASop));
    oper_counter += 1;

    // LAX immediate (ATX, LXA, OAL) operation
    ops[0xAB] = Operation::new(2, immediate, CPUInstByte::Two(Inst2Byte::LAX2op));
    oper_counter += 1;

    // LAS (LAR, LAE) operations
    ops[0xBB] = Operation::new(4, absolute_y, CPUInstByte::Three(Inst3Byte::LASop));
    oper_counter += 1;

    let mut op_id: usize = 0;
    while op_id <= (u8::MAX) as usize {
        let mut op = ops[op_id];
        op.code = op_id as u8;
        ops[op_id] = op;
        op_id += 1;
    }

    (ops, oper_counter)
}

impl std::fmt::Display for Inst1Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Inst1Byte::TAXop => write!(f, "TAX"),
            Inst1Byte::TAYop => write!(f, "TAY"),
            Inst1Byte::TXAop => write!(f, "TXA"),
            Inst1Byte::TYAop => write!(f, "TYA"),
            Inst1Byte::TSXop => write!(f, "TSX"),
            Inst1Byte::TXSop => write!(f, "TXS"),
            Inst1Byte::PHAop => write!(f, "PHA"),
            Inst1Byte::PHPop => write!(f, "PHP"),
            Inst1Byte::PLAop => write!(f, "PLA"),
            Inst1Byte::PLPop => write!(f, "PLP"),
            Inst1Byte::INXop => write!(f, "INX"),
            Inst1Byte::INYop => write!(f, "INY"),
            Inst1Byte::DEXop => write!(f, "DEX"),
            Inst1Byte::DEYop => write!(f, "DEY"),
            Inst1Byte::ASLop => write!(f, "ASL"),
            Inst1Byte::LSRop => write!(f, "LSR"),
            Inst1Byte::ROLop => write!(f, "ROL"),
            Inst1Byte::RORop => write!(f, "ROR"),
            Inst1Byte::RTSop => write!(f, "RTS"),
            Inst1Byte::CLCop => write!(f, "CLC"),
            Inst1Byte::CLDop => write!(f, "CLD"),
            Inst1Byte::CLIop => write!(f, "CLI"),
            Inst1Byte::CLVop => write!(f, "CLV"),
            Inst1Byte::SECop => write!(f, "SEC"),
            Inst1Byte::SEDop => write!(f, "SED"),
            Inst1Byte::SEIop => write!(f, "SEI"),
            Inst1Byte::BRKop => write!(f, "BRK"),
            Inst1Byte::NOPop => write!(f, "NOP"),
            Inst1Byte::RTIop => write!(f, "RTI"),
            Inst1Byte::STPop => write!(f, "STP"),
        }
    }
}

impl std::fmt::Display for Inst2Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Inst2Byte::LDAop => write!(f, "LDA"),
            Inst2Byte::LDXop => write!(f, "LDX"),
            Inst2Byte::LDYop => write!(f, "LDY"),
            Inst2Byte::STAop => write!(f, "STA"),
            Inst2Byte::STXop => write!(f, "STX"),
            Inst2Byte::STYop => write!(f, "STY"),
            Inst2Byte::ANDop => write!(f, "AND"),
            Inst2Byte::EORop => write!(f, "EOR"),
            Inst2Byte::ORAop => write!(f, "ORA"),
            Inst2Byte::BITop => write!(f, "BIT"),
            Inst2Byte::ADCop => write!(f, "ADC"),
            Inst2Byte::SBCop => write!(f, "SBC"),
            Inst2Byte::CMPop => write!(f, "CMP"),
            Inst2Byte::CPXop => write!(f, "CPX"),
            Inst2Byte::CPYop => write!(f, "CPY"),
            Inst2Byte::INCop => write!(f, "INC"),
            Inst2Byte::DECop => write!(f, "DEC"),
            Inst2Byte::ASLop => write!(f, "ASL"),
            Inst2Byte::LSRop => write!(f, "LSR"),
            Inst2Byte::ROLop => write!(f, "ROL"),
            Inst2Byte::RORop => write!(f, "ROR"),
            Inst2Byte::BCCop => write!(f, "BCC"),
            Inst2Byte::BCSop => write!(f, "BCS"),
            Inst2Byte::BEQop => write!(f, "BEQ"),
            Inst2Byte::BMIop => write!(f, "BMI"),
            Inst2Byte::BNEop => write!(f, "BNE"),
            Inst2Byte::BPLop => write!(f, "BPL"),
            Inst2Byte::BVCop => write!(f, "BVC"),
            Inst2Byte::BVSop => write!(f, "BVS"),
            Inst2Byte::ALRop => write!(f, "ALR"),
            Inst2Byte::ANCop => write!(f, "ANC"),
            Inst2Byte::ARRop => write!(f, "ARR"),
            Inst2Byte::AXSop => write!(f, "AXS"),
            Inst2Byte::LAXop => write!(f, "LAX"),
            Inst2Byte::SAXop => write!(f, "SAX"),
            Inst2Byte::DCPop => write!(f, "DCP"),
            Inst2Byte::ISCop => write!(f, "ISC"),
            Inst2Byte::RLAop => write!(f, "RLA"),
            Inst2Byte::RRAop => write!(f, "RRA"),
            Inst2Byte::SLOop => write!(f, "SLO"),
            Inst2Byte::SREop => write!(f, "SRE"),
            Inst2Byte::NOPop => write!(f, "NOP"),
            Inst2Byte::XAAop => write!(f, "XAA"),
            Inst2Byte::AHXop => write!(f, "AHX"),
            Inst2Byte::LAX2op => write!(f, "LAX2"),
        }
    }
}

impl std::fmt::Display for Inst3Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Inst3Byte::LDAop => write!(f, "LDA"),
            Inst3Byte::LDXop => write!(f, "LDX"),
            Inst3Byte::LDYop => write!(f, "LDY"),
            Inst3Byte::STAop => write!(f, "STA"),
            Inst3Byte::STXop => write!(f, "STX"),
            Inst3Byte::STYop => write!(f, "STY"),
            Inst3Byte::ANDop => write!(f, "AND"),
            Inst3Byte::EORop => write!(f, "EOR"),
            Inst3Byte::ORAop => write!(f, "ORA"),
            Inst3Byte::BITop => write!(f, "BIT"),
            Inst3Byte::ADCop => write!(f, "ADC"),
            Inst3Byte::SBCop => write!(f, "SBC"),
            Inst3Byte::CMPop => write!(f, "CMP"),
            Inst3Byte::CPXop => write!(f, "CPX"),
            Inst3Byte::CPYop => write!(f, "CPY"),
            Inst3Byte::INCop => write!(f, "INC"),
            Inst3Byte::DECop => write!(f, "DEC"),
            Inst3Byte::ASLop => write!(f, "ASL"),
            Inst3Byte::LSRop => write!(f, "LSR"),
            Inst3Byte::ROLop => write!(f, "ROL"),
            Inst3Byte::RORop => write!(f, "ROR"),
            Inst3Byte::JMPop => write!(f, "JMP"),
            Inst3Byte::JSRop => write!(f, "JSR"),
            Inst3Byte::LAXop => write!(f, "LAX"),
            Inst3Byte::SAXop => write!(f, "SAX"),
            Inst3Byte::DCPop => write!(f, "DCP"),
            Inst3Byte::ISCop => write!(f, "ISC"),
            Inst3Byte::RLAop => write!(f, "RLA"),
            Inst3Byte::RRAop => write!(f, "RRA"),
            Inst3Byte::SLOop => write!(f, "SLO"),
            Inst3Byte::SREop => write!(f, "SRE"),
            Inst3Byte::SHXop => write!(f, "SHX"),
            Inst3Byte::SHYop => write!(f, "SHY"),
            Inst3Byte::NOPop => write!(f, "NOP"),
            Inst3Byte::AHXop => write!(f, "AHX"),
            Inst3Byte::TASop => write!(f, "TAS"),
            Inst3Byte::LASop => write!(f, "LAS"),
        }
    }
}

impl std::fmt::Display for CPUInstByte {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CPUInstByte::One(inst) => write!(f, "{inst}"),
            CPUInstByte::Two(inst) => write!(f, "{inst}"),
            CPUInstByte::Three(inst) => write!(f, "{inst}"),
            CPUInstByte::NoOp => write!(f, "NoOp"),
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} at {}", self.op_name, self.memory_type)
    }
}

impl Operation {
    pub fn raw_disassembly(&self, pc: u16, bytes: &[u8]) -> String {
        let mut op_disassembly = format!("{}", self.op_name);
        if !bytes.is_empty() {
            op_disassembly.push_str(&format!(" {}", self.memory_type.dispay_bytes(bytes, pc)));
        }

        op_disassembly
    }
}
