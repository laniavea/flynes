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

impl PpuCtrlSettings {
    fn base_nametables_addr_to_t_reg(&self) -> u16 {
        match self.base_nametables_addr {
            0x2000 => 0b0000_0000_0000_0000,
            0x2400 => 0b0100_0000_0000_0000,
            0x2800 => 0b1000_0000_0000_0000,
            0x2C00 => 0b1100_0000_0000_0000,
            _ => unreachable!("unreachable because of mod 4")
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PpuMaskSetting {
    emphasize_blue: bool,
    emphasize_green: bool,
    emphasize_red: bool,
    sprite_render: bool,
    bg_render: bool,
    leftmost_sprite_render: bool,
    leftmost_bg_render: bool,
    greyscale: bool,
}

impl PpuMaskSetting {
    fn set(&mut self, settings: u8) {
        self.emphasize_blue = is_bit_set(settings, 0b1000_0000);
        self.emphasize_green = is_bit_set(settings, 0b0100_0000);
        self.emphasize_red = is_bit_set(settings, 0b0010_0000);
        self.sprite_render = is_bit_set(settings, 0b0001_0000);
        self.bg_render = is_bit_set(settings, 0b0000_1000);
        self.leftmost_sprite_render = is_bit_set(settings, 0b0000_0100);
        self.leftmost_bg_render = is_bit_set(settings, 0b0000_0010);
        self.greyscale = is_bit_set(settings, 0b0000_0001);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PpuStatus {
    value: u8
}

impl PpuStatus {
    pub fn set_v_blank(&mut self) {
        self.value |= 0b1000_0000
    }

    pub fn clear_v_blank(&mut self) {
        self.value &= 0b0111_1111
    }

    pub fn set_sprite_zero_hit(&mut self) {
        self.value |= 0b0100_0000
    }

    pub fn clear_sprite_zero_hit(&mut self) {
        self.value &= 0b1011_1111
    }

    pub fn set_sprite_overflow(&mut self) {
        self.value |= 0b0010_0000
    }

    pub fn clear_sprite_overflow(&mut self) {
        self.value &= 0b1101_1111
    }

    pub fn open_bus_write(&mut self, value_to_write: u8) {
        self.value &= 0b1110_0000;
        self.value |= 0b0001_1111 & value_to_write;
    }
}


#[derive(Debug, Default, Clone)]
pub struct Ppu {
    registers: [u8; 9],
    t_register: u16,
    write_toogle: bool,
    x_scroll: u8,
    y_scroll: u8,
    ctrl_settings: PpuCtrlSettings,
    render_settings: PpuMaskSetting,
    ppu_status: PpuStatus,
}

impl Ppu {
    pub fn write_to_registers(&mut self, register: usize, data: u8) {
        inst_assert!((0..=8).contains(&register));
        match register {
            PPU_CTRL_REG => {
                self.registers[PPU_CTRL_REG] = data;
                self.ctrl_settings.set(data);
                self.t_register &= 0b0011_1111_1111_1111;
                self.t_register |= self.ctrl_settings.base_nametables_addr_to_t_reg();
            },
            PPU_MASK_REG => {
                self.registers[PPU_MASK_REG] = data;
                self.render_settings.set(data);
            },
            PPU_STATUS_REG => {
                self.ppu_status.open_bus_write(data);
                warn!("Trying to write to $2002, which is read only, open bus write");
            },
            OAM_ADDR_REG => {
                self.registers[OAM_ADDR_REG] = data;
            },
            OAM_DATA_REG => {
                self.registers[OAM_ADDR_REG] = self.registers[OAM_ADDR_REG].wrapping_add(1);
                self.registers[OAM_DATA_REG] = data;
            },
            PPU_SCROLL_REG => {
                if !self.write_toogle { // First write, w is 0 (false)
                    self.x_scroll = data;
                } else {
                    self.y_scroll = data;
                }
                self.registers[PPU_SCROLL_REG] = data;
            },
            PPU_ADDR_REG => {
                if !self.write_toogle { // First write, w is 0 (false)
                    self.t_register = ((data & 0b0011_1111) as u16) << 8;
                    self.write_toogle = true;
                } else {
                    self.t_register += data as u16;
                    self.write_toogle = false;
                }
                self.registers[PPU_ADDR_REG] = data;
            },
            PPU_DATA_REG => {
                self.registers[register] = data;
            },
            OAM_DMA_REG => {
                self.registers[register] = data;
            },
            _ => unreachable!("No more registers")
        }

    }

    pub fn read_from_registers(&mut self, register: usize) -> u8 {
        inst_assert!((0..=8).contains(&register));
        match register {
            0 | 1 | 3 | 5 | 6 | 8 => {
                warn!("Trying to read from 0x200{register}, which is write only, ret 0");
                0
            },
            PPU_STATUS_REG => {
                self.write_toogle = false;
                self.registers[PPU_STATUS_REG] = self.ppu_status.value;
                self.registers[PPU_STATUS_REG]
            },
            OAM_DATA_REG => {
                self.registers[OAM_DATA_REG]
            },
            PPU_DATA_REG => {
                self.registers[PPU_DATA_REG]
            },
            _ => unreachable!("No more registers")
        }
    }
}
