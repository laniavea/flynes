use crate::mappers::MapperRW;

#[derive(Debug, Clone)]
pub struct NoMapper {
}

impl NoMapper {
    pub fn init(_prg_rom: &[u8], _prg_data: &mut [u8]) -> NoMapper {
        NoMapper { }
    }
}

impl MapperRW for NoMapper {
    fn read(&self, _data_ref: usize, _prg_data: &[u8]) -> u8 {
        unreachable!("Trying to use NoMapper (CPU read)");
    }

    fn write(&self, _data_ref: usize, _value: u8, _prg_data: &mut [u8]) {
        unreachable!("Trying to use NoMapper (CPU write)");
    }

    fn read_ppu(&self, _data_ref: usize, _chr_data: &[u8]) -> u8 {
        unreachable!("Trying to use NoMapper (PPU read)");
    }
}
