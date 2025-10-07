use better_assertions::inst_assert;
use log::warn;

use crate::mappers::create_ram;
use crate::mappers::{Mappers, MapperRW, MappersError};
use crate::common::DataSizes;

const PRG_ROM_CAPACITY: [usize; 2] = [DataSizes::Size16K.to_bytes(), DataSizes::Size32K.to_bytes()];
const PRG_RAM_CAPACITY: usize = DataSizes::Size8K.to_bytes();
const PRG_RAM_HW_START: usize = 0x6000;

#[derive(Debug, Clone)]
pub struct NROM {
    prg_rom_start: usize,
    prg_rom_mirror: bool,
}

impl NROM {
    pub fn init(prg_rom: &[u8], prg_data: &mut Vec<u8>) -> Result<Mappers, MappersError> {
        if !PRG_ROM_CAPACITY.contains(&prg_rom.len()) {
            return Err(MappersError::IncorrectSizePRGROM);
        }
        let prg_rom_mirror = prg_rom.len() == DataSizes::Size16K.to_bytes();

        create_ram(prg_data, PRG_RAM_CAPACITY);
        prg_data.extend_from_slice(prg_rom);

        Ok(Mappers::NROM(
            NROM {
                prg_rom_start: PRG_RAM_CAPACITY,
                prg_rom_mirror
            }
        ))
    }
}

impl MapperRW for NROM {
    fn mapper_read(&self, data_ref: usize, prg_data: &[u8]) -> u8 {
        inst_assert!((0x4020..=0xFFFF).contains(&data_ref));

        if data_ref >= PRG_RAM_HW_START + PRG_RAM_CAPACITY {
            let mut shifted_ref = data_ref - (PRG_RAM_HW_START + PRG_RAM_CAPACITY);
            if self.prg_rom_mirror && shifted_ref >= DataSizes::Size16K.to_bytes() {
                shifted_ref -= DataSizes::Size16K.to_bytes();
            }
            prg_data[self.prg_rom_start + shifted_ref]

        } else if data_ref >= PRG_RAM_HW_START {
            prg_data[data_ref - PRG_RAM_HW_START]

        } else {
            warn!("Trying to read from expanstion ROM which doesn't exists, ret 0");
            0
        }
    }

    fn mapper_write(&self, data_ref: usize, value: u8, prg_data: &mut [u8]) {
        inst_assert!((0x4020..=0xFFFF).contains(&data_ref));

        if data_ref < PRG_RAM_HW_START {
            warn!("Trying to write to expanstion ROM which doesn't exists");
            return
        }

        let shifted_ref = data_ref - PRG_RAM_HW_START;
        if shifted_ref >= PRG_RAM_CAPACITY {
            warn!("Trying to write to PRG-ROM, ignored");
            return
        }

        prg_data[shifted_ref] = value;
    }
}
