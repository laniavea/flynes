use better_assertions::inst_assert;

use crate::bus::AddressCpu;
use crate::cpu::instructions::Operation;
use crate::memory::MemoryType;

#[derive(Clone, Debug, Copy)]
pub enum BusDataSource {
    RAM(u16),
    PpuRegs(u16),
    ApuRegs(u16),
    Mapper(u16),
}

#[derive(Clone, Debug, Copy)]
pub enum BusRWInfo {
    Read(u8),
    Write((u8, u8)),
}

#[derive(Clone, Debug, Copy, Default)]
pub struct DecodingDebug {
    register_value: Option<u8>,
    sub_result: Option<u16>,
    result: Option<u16>,
}

#[derive(Clone, Debug, Copy)]
pub struct CpuDebug {
    pc: u16,
    operation: Operation,
    decoding_info: DecodingDebug,
    first_byte: Option<u8>,
    second_byte: Option<u8>,
}

#[derive(Clone, Debug, Copy)]
pub struct BusOperationInfo {
    cycles: usize,
    address: u16,
    data_source: BusDataSource,
    op_info: BusRWInfo,
}

#[derive(Clone, Debug)]
pub struct BusDebug {
    fetch_len: u8,
    decode_len: Option<u8>,
    operations: Vec<BusOperationInfo>,
}

impl BusDebug {
    pub fn clear_operations(&mut self) {
        self.fetch_len = 0;
        self.decode_len = None;
        self.operations.clear()
    }

    pub fn add_op(&mut self, bus_operation_info: BusOperationInfo) {
        self.operations.push(bus_operation_info);
    }

    pub fn end_fetch(&mut self) {
        inst_assert!(self.operations.len() <= 255);
        self.fetch_len = self.operations.len() as u8;
    }

    pub fn end_decode(&mut self) {
        inst_assert!(self.fetch_len > 0);
        if self.operations.len() > self.fetch_len as usize {
            inst_assert!(self.operations.len() <= 255);
            self.decode_len = Some((self.operations.len() as u8) - self.fetch_len);
            inst_assert!(self.operations.len() > self.decode_len.unwrap() as usize);
        }
    }

    pub fn get_fetch(&self) -> &[BusOperationInfo] {
        inst_assert!(self.fetch_len > 0);
        let fetch_end_id = self.fetch_len as usize;
        &self.operations[..fetch_end_id]
    }

    pub fn get_decode(&self) -> Option<&[BusOperationInfo]> {
        inst_assert!(self.fetch_len > 0);
        if let Some(decode_len) = self.decode_len {
            inst_assert!(decode_len > 0);
            let lower_bound = self.fetch_len as usize;
            let upper_bound = lower_bound + (decode_len as usize);
            Some(&self.operations[lower_bound..upper_bound])
        } else {
            None
        }
    }

    pub fn get_execute(&self) -> Option<&[BusOperationInfo]> {
        inst_assert!(self.fetch_len > 0);
        if let Some(decode_len) = self.decode_len {
            inst_assert!(decode_len > 0);
            let lower_bound = (self.fetch_len + decode_len) as usize;

            if lower_bound == self.operations.len() {
                return None;
            }
            Some(&self.operations[lower_bound..])
        } else {
            if self.fetch_len as usize == self.operations.len() {
                return None;
            }
            Some(&self.operations[(self.fetch_len as usize)..])
        }
    }

    pub fn operations(&self) -> &Vec<BusOperationInfo> {
        &self.operations
    }

    pub fn new() -> BusDebug {
        BusDebug {
            operations: Vec::with_capacity(8),
            fetch_len: 0,
            decode_len: None,
        }
    }
}

impl Default for BusDebug {
    fn default() -> BusDebug {
        BusDebug::new()
    }
}

impl CpuDebug {
    pub fn new(pc: u16, operation: Operation) -> Self {
        CpuDebug {
            pc,
            operation,
            decoding_info: DecodingDebug::default(),
            first_byte: None,
            second_byte: None,
        }
    }

    pub fn set_first_byte(&mut self, byte_to_set: u8) {
        self.first_byte = Some(byte_to_set);
    }

    pub fn set_second_byte(&mut self, byte_to_set: u8) {
        self.second_byte = Some(byte_to_set);
    }

    pub fn set_decoding_register_val(&mut self, reg: u8) {
        self.decoding_info.register_value = Some(reg);
    }

    pub fn set_decoding_sub_res(&mut self, value: u16) {
        self.decoding_info.sub_result = Some(value);
    }

    pub fn set_decoding_result(&mut self, decoded_address: u16) {
        self.decoding_info.result = Some(decoded_address);
    }
}

impl BusOperationInfo {
    pub fn new(
        cycles: usize,
        address: u16,
        data_source: BusDataSource,
        op_info: BusRWInfo,
    ) -> BusOperationInfo {
        BusOperationInfo {
            cycles,
            address,
            data_source,
            op_info,
        }
    }
}

impl From<AddressCpu> for BusDataSource {
    fn from(val: AddressCpu) -> Self {
        match val {
            AddressCpu::RAM(address) => BusDataSource::RAM(address as u16),
            AddressCpu::PpuRegs(address) => BusDataSource::PpuRegs(address as u16),
            AddressCpu::ApuRegs(address) => BusDataSource::ApuRegs(address as u16),
            AddressCpu::ROM(_address) => BusDataSource::Mapper(0), //FIX: Add rom bank for mapper data (0 -> ?)
        }
    }
}

impl CpuDebug {
    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn operation(&self) -> Operation {
        self.operation
    }

    pub fn first_byte(&self) -> Option<u8> {
        self.first_byte
    }

    pub fn second_byte(&self) -> Option<u8> {
        self.second_byte
    }

    pub fn fetched_bytes(&self) -> Vec<u8> {
        let mut op_bytes = vec![self.operation.code()];
        if let Some(fb) = self.first_byte {
            op_bytes.push(fb);
        }
        if let Some(sb) = self.second_byte {
            op_bytes.push(sb);
        }
        op_bytes
    }
}

impl CpuDebug {
    fn disassembly_1_byte_addressing(&self) -> String {
        let data_byte = self.first_byte.unwrap();
        match self.operation.memory_type() {
            MemoryType::Immediate => {
                format!("#${}", crate::common::number_to_hex(data_byte, false))
            }
            MemoryType::ZeroPage => {
                format!("${}", crate::common::number_to_hex(data_byte, false))
            }
            MemoryType::ZeroPageX => {
                format!(
                    "${},X @ {}",
                    crate::common::number_to_hex(data_byte, false),
                    crate::common::number_to_hex(self.decoding_info.register_value.unwrap(), false)
                )
            }
            MemoryType::ZeroPageY => {
                format!(
                    "${},Y @ {}",
                    crate::common::number_to_hex(data_byte, false),
                    crate::common::number_to_hex(self.decoding_info.register_value.unwrap(), false)
                )
            }
            MemoryType::Relative => {
                let pc_offset = crate::cpu::instructions::shared_ops::branch_pc_calc(
                    self.pc.wrapping_add(1).wrapping_add(1),
                    data_byte,
                );
                format!("${}", crate::common::number_to_hex(pc_offset, false))
            }
            MemoryType::IndirectX => {
                format!(
                    "(${},X) @ {} = {}",
                    crate::common::number_to_hex(data_byte, false),
                    crate::common::number_to_hex(self.decoding_info.register_value.unwrap(), false),
                    crate::common::number_to_hex(self.decoding_info.result.unwrap(), false)
                )
            }
            MemoryType::IndirectY => {
                format!(
                    "(${}),Y = {} @ {}",
                    crate::common::number_to_hex(data_byte, false),
                    crate::common::number_to_hex(self.decoding_info.sub_result.unwrap(), false),
                    crate::common::number_to_hex(self.decoding_info.result.unwrap(), false)
                )
            }
            _ => unreachable!(
                "Disassembly 1 data byte was called for incorrect memory type: {}",
                self.operation.memory_type()
            ),
        }
    }

    fn disassembly_2_bytes_addressing(&self) -> String {
        let first_byte = self.first_byte.unwrap();
        let second_byte = self.second_byte.unwrap();
        let converted_address = crate::common::number_to_hex(
            crate::cpu::Cpu::bytes_to_16bit_le_order(first_byte, second_byte),
            false,
        );

        match self.operation.memory_type() {
            MemoryType::Absolute => format!("${}", converted_address),
            MemoryType::AbsoluteX => {
                format!(
                    "${},X @ {}",
                    converted_address,
                    crate::common::number_to_hex(self.decoding_info.result.unwrap(), false)
                )
            }
            MemoryType::AbsoluteY => {
                format!(
                    "${},Y @ {}",
                    converted_address,
                    crate::common::number_to_hex(self.decoding_info.result.unwrap(), false)
                )
            }
            MemoryType::Indirect => {
                format!(
                    "(${}) = {}",
                    converted_address,
                    crate::common::number_to_hex(self.decoding_info.result.unwrap(), false)
                )
            }
            _ => unreachable!(
                "Disassembly 2 data bytes was called for incorrect memory type: {}",
                self.operation.memory_type()
            ),
        }
    }

    pub fn disassembly_addressing(&self) -> String {
        if self.first_byte.is_some() {
            if self.second_byte.is_some() {
                self.disassembly_2_bytes_addressing()
            } else {
                self.disassembly_1_byte_addressing()
            }
        } else {
            inst_assert!(self.second_byte.is_none());
            "".to_string()
        }
    }

    pub fn get_inst_and_addressing(&self) -> String {
        let mut op_disassembly = if self.operation.memory_type() == MemoryType::Accumulator {
            format!("{} A", self.operation.op_name())
        } else {
            format!("{}", self.operation.op_name())
        };
        if self.first_byte.is_some() {
            op_disassembly.push_str(format!(" {}", self.disassembly_addressing()).as_str());
        }
        op_disassembly
    }
}

impl BusOperationInfo {
    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn address(&self) -> u16 {
        self.address
    }

    pub fn data_source(&self) -> BusDataSource {
        self.data_source
    }

    pub fn op_info(&self) -> BusRWInfo {
        self.op_info
    }
}

pub fn cpu_debug_str(cpu_debug_info: &CpuDebug, bus_debug: &BusDebug) -> String {
    let mut res = cpu_debug_info.get_inst_and_addressing();

    let execution_bus_ops = bus_debug.get_execute();
    if let Some(execution_ops) = execution_bus_ops {
        let execution_op = execution_ops.last().unwrap();
        match execution_op.op_info {
            BusRWInfo::Write((byte_before, _byte_written)) => {
                res.push_str(&format!(
                    " = {}",
                    crate::common::number_to_hex(byte_before, false)
                ));
            }
            BusRWInfo::Read(byte_readed) => {
                const SPECIAL_MTS: [MemoryType; 8] = [
                    MemoryType::ZeroPage,
                    MemoryType::Absolute,
                    MemoryType::IndirectX,
                    MemoryType::IndirectY,
                    MemoryType::AbsoluteY,
                    MemoryType::AbsoluteX,
                    MemoryType::ZeroPageX,
                    MemoryType::ZeroPageY,
                ];
                if SPECIAL_MTS.contains(&cpu_debug_info.operation.memory_type()) {
                    res.push_str(&format!(
                        " = {}",
                        crate::common::number_to_hex(byte_readed, false)
                    ));
                }
            }
        }
    }

    res
}
