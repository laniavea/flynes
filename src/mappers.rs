use better_assertions::inst_assert_eq;
use enum_dispatch::enum_dispatch;

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
}

#[derive(Debug, Clone, Copy)]
pub enum MappersError {
    IncorrectSizePRGROM
}

impl std::fmt::Display for MappersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IncorrectSizePRGROM => write!(f, "Number of bytes provided from NES file doesn't apply to mapper"),
        }
    }
}

pub fn create_ram(prg_data: &mut Vec<u8>, num: usize) {
    inst_assert_eq!(prg_data.len(), 0);
    *prg_data = vec![0u8; num];
}

pub fn create_mapper(mapper_type: u8, prg_rom: &[u8]) -> Result<(Vec<u8>, Mappers), MappersError> {
    let mut prg_data = Vec::new();
    let mapper = match mapper_type {
        0 => NROM::init(prg_rom, &mut prg_data)?,
        _ => unimplemented!("Other mappers unimplemented")
    };

    Ok((prg_data, mapper))
}
