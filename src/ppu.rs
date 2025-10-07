use better_assertions::inst_assert;

const _PPU_CTRL_REG: usize = 0;
const _PPU_MASK_REG: usize = 1;
const _PPU_STATUS_REG: usize = 2;
const _OAM_ADDR_REG: usize = 3;
const _OAM_DATA_REG: usize = 4;
const _PPU_SCROLL_REG: usize = 5;
const _PPU_ADDR_REG: usize = 6;
const _PPU_DATA_REG: usize = 7;
const _OAM_DMA_REG: usize = 8;

#[derive(Debug, Default, Clone)]
pub struct Ppu {
    registers: [u8; 9],
}

impl Ppu {
    pub fn write_to_registers(&mut self, register: usize, data: u8) {
        inst_assert!((0..=8).contains(&register));
        self.registers[register] = data;
    }

    pub fn read_from_registers(&self, register: usize) -> u8 {
        inst_assert!((0..=8).contains(&register));
        self.registers[register]
    }
}
