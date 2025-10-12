use log::warn;
use better_assertions::inst_assert;

use crate::common::is_bit_set;

const PPU_CTRL_REG: usize = 0;
const PPU_MASK_REG: usize = 1;
const PPU_STATUS_REG: usize = 2;
const OAM_ADDR_REG: usize = 3;
const OAM_DATA_REG: usize = 4;
const PPU_SCROLL_REG: usize = 5;
const PPU_ADDR_REG: usize = 6;
const PPU_DATA_REG: usize = 7;
const OAM_DMA_REG: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct PpuCtrlSettings {
    v_blank_nmi: bool,
    master_slave_mode: bool,
    sprite_size: u8,
    bg_addr: u16,
    sprite_pt_address: u16,
    vram_address_inc: u16,
    base_nametables_addr: u16,
}

impl Default for PpuCtrlSettings {
    fn default() -> Self {
        Self {
            v_blank_nmi: false,
            master_slave_mode: false,
            sprite_size: 8,
            bg_addr: 0x0000,
            sprite_pt_address: 0x0000,
            vram_address_inc: 1,
            base_nametables_addr: 0x2000,
        }
    }
}

impl PpuCtrlSettings {
    fn set(&mut self, settings: u8) {
        self.v_blank_nmi = is_bit_set(settings, 0b1000_0000);
        self.master_slave_mode = is_bit_set(settings, 0b0100_0000);

        self.sprite_size = if is_bit_set(settings, 0b0010_0000) {
            16
        } else {
            8
        };

        self.bg_addr = if is_bit_set(settings, 0b0001_0000) {
            0x1000
        } else {
            0x0000
        };

        self.sprite_pt_address = if is_bit_set(settings, 0b0000_1000) {
            0x1000
        } else {
            0x0000
        };

        self.vram_address_inc = if is_bit_set(settings, 0b0000_0100) {
            32
        } else {
            1
        };

        self.base_nametables_addr = match settings % 4 {
            0 => 0x2000, // ...00
            1 => 0x2400, // ...01
            2 => 0x2800, // ...10
            3 => 0x2C00, // ...11
            _ => unreachable!("unreachable because of mod 4")
        };
    }
}

#[derive(Debug, Default, Clone)]
pub struct Ppu {
    registers: [u8; 9],
    t_register: u16,
    write_toogle: bool,
    ctrl_settings: PpuCtrlSettings,

}

impl Ppu {
    pub fn write_to_registers(&mut self, register: usize, data: u8) {
        inst_assert!((0..=8).contains(&register));
        match register {
            PPU_CTRL_REG => {
                self.registers[register] = data;
                self.ctrl_settings.set(data);
            },
            PPU_MASK_REG => {
                self.registers[register] = data;
            },
            PPU_STATUS_REG => {
                warn!("Trying to write to $2002, which is read only, ignored");
            },
            OAM_ADDR_REG => {
                self.registers[register] = data;
            },
            OAM_DATA_REG => {
                self.registers[register] = data;
            },
            PPU_SCROLL_REG => {
                self.registers[register] = data;
            },
            PPU_ADDR_REG => {
                if !self.write_toogle { // First write, w is 0 (false)
                    self.t_register = ((data & 0b0011_1111) as u16) << 8;
                    self.write_toogle = true;
                } else {
                    self.t_register += data as u16;
                    self.write_toogle = false;
                }
                self.registers[register] = data;
            },
            PPU_DATA_REG => {
                self.registers[PPU_ADDR_REG] = self.registers[PPU_ADDR_REG].wrapping_add(0);
                self.registers[register] = data;
            },
            OAM_DMA_REG => {
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
            PPU_STATUS_REG => {
                self.registers[register]
            },
            OAM_DATA_REG => {
                self.registers[register]
            },
            PPU_DATA_REG => {
                self.registers[register]
            },
            _ => unreachable!("No more registers")
        }
    }
}
