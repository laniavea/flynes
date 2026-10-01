use std::fmt::Debug;

use crate::cpu::instructions::Operation;
use crate::bus::AddressCpu;

#[derive(Clone, Debug, Copy)]
pub enum BusDataSource {
    RAM(u16),
    PpuRegs(u16),
    ApuRegs(u16),
    Mapper(u16),
}

#[derive(Clone, Debug, Copy)]
pub enum BusOperationInfo {
    Read(u8),
    Write(u8),
}

#[derive(Clone, Debug, Copy)]
pub struct CpuDebugInfo {
    pc: u16,
    operation: Operation,
    first_byte: Option<u8>,
    second_byte: Option<u8>,
}

#[derive(Clone, Debug)]
pub struct BusDebugInfo {
    cycles: usize,
    address: u16,
    data_source: BusDataSource,
    op_info: BusOperationInfo,
}

impl CpuDebugInfo {
    pub fn new(
        pc: u16,
        operation: Operation,
        first_byte: Option<u8>,
        second_byte: Option<u8>,
    ) -> Self {
        CpuDebugInfo {
            pc,
            operation,
            first_byte,
            second_byte,
        }
    }
}

impl BusDebugInfo {
    pub fn new(
        cycles: usize,
        address: u16,
        data_source: BusDataSource,
        op_info: BusOperationInfo,
    ) -> BusDebugInfo {
        BusDebugInfo {
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

impl CpuDebugInfo {
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

    pub fn as_bytes(&self) -> Vec<u8> {
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

impl BusDebugInfo {
    pub fn cycles(&self) -> usize {
        self.cycles
    }

    pub fn address(&self) -> u16 {
        self.address
    }

    pub fn data_source(&self) -> BusDataSource {
        self.data_source
    }

    pub fn op_info(&self) -> BusOperationInfo {
        self.op_info
    }
}
