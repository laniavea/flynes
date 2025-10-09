use better_assertions::inst_assert_eq;
use enum_dispatch::enum_dispatch;

use crate::memory::Memory;
use no_mapper::NoMapper;
use nrom::NROM;

pub mod no_mapper;
pub mod nrom;

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum Mappers {
    NoMapper,
    NROM,
}

impl Default for Mappers {
    fn default() -> Self {
        Mappers::NoMapper(NoMapper { })
    }
}

#[enum_dispatch(Mappers)]
pub trait MapperRW {
    fn mapper_read(&self, req_addr: usize, prg_data: &[u8]) -> u8;
    fn mapper_write(&self, req_addr: usize, value: u8, prg_data: &mut [u8]);
    fn mapper_read_ppu(&self, data_ref: usize, chr_data: &[u8]) -> u8;
}

#[derive(Debug, Clone, Copy)]
pub enum MappersError {
    IncorrectSizePRGROM,
    IncorrectSizeCHRROM,
}

impl std::fmt::Display for MappersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IncorrectSizePRGROM => write!(f, "Size of PRG-ROM from NES file doesn't apply to mapper"),
            Self::IncorrectSizeCHRROM => write!(f, "Size of CHR-ROM from NES file doesn't apply to mapper"),
        }
    }
}

pub fn fill_ram(prg_data: &mut Vec<u8>, num: usize) {
    inst_assert_eq!(prg_data.len(), 0);
    *prg_data = vec![0u8; num];
}

pub fn create_mapper(mapper_type: u8, mem_module: &mut Memory, prg_rom: &[u8], chr_rom: &[u8]) -> Result<Mappers, MappersError> {
    let mapper = match mapper_type {
        0 => NROM::init(mem_module, prg_rom, chr_rom)?,
        _ => unimplemented!("Other mappers unimplemented")
    };

    Ok(mapper)
}
