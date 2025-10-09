use log::warn;
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
        match register {
            0 => {
                self.registers[register] = data;
            },
            1 => {
                self.registers[register] = data;
            },
            2 => {
                warn!("Trying to write to $2002, which is read only, ignored");
            },
            3 => {
                self.registers[register] = data;
            },
            4 => {
                self.registers[register] = data;
            },
            5 => {
                self.registers[register] = data;
            },
            6 => {
                self.registers[register] = data;
            },
            7 => {
                self.registers[register] = data;
            },
            8 => {
                self.registers[register] = data;
            },
            _ => unreachable!("No more registers")
        }

    }

    pub fn read_from_registers(&self, register: usize) -> u8 {
        inst_assert!((0..=8).contains(&register));
        match register {
            0 | 1 | 3 | 5 | 6 | 8 => {
                warn!("Trying to read from 0x200{register}, which is write only, ret 0");
                0
            },
            2 => {
                self.registers[register]
            },
            4 => {
                self.registers[register]
            },
            7 => {
                self.registers[register]
            },
            _ => unreachable!("No more registers")
        }
    }
}
