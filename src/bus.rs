use better_assertions::inst_assert;
use log::warn;

use crate::memory::Memory;
use crate::memory::{PPU_REGS_MIRRORS, APU_REGS, APU_IO_FUNC, PPU_REGS, RAM_MIRRORS, RAM, EXPANSION_ROM};
use crate::memory::{PPU_PATTERN_TABLES, PPU_NAME_TABLES, PPU_UNUSED_SPACE, PPU_PALETTES};
use crate::ppu::Ppu;
use crate::mappers::{Mappers, MapperRW};

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory,
    ppu: Ppu,
    mapper: Mappers,
    cpu_cycles_num: usize,
}

impl Bus {
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn set_mapper(&mut self, mapper: Mappers) {
        self.mapper = mapper;
    }
}

impl Bus {
    pub fn read_8bit_cpu<T>(&mut self, requested_address: T, actual_cpu_cycles: &usize) -> u8
    where 
        T: Into<usize> + Copy
    {
        let requested_address: usize = requested_address.into();

        if requested_address > EXPANSION_ROM.start {
            inst_assert!((EXPANSION_ROM.start..=(u16::MAX as usize)).contains(&requested_address));
            self.mapper.read(requested_address, self.memory.prg_data())
        } else if requested_address >= APU_REGS.start {
            inst_assert!((APU_REGS.start..=APU_IO_FUNC.end).contains(&requested_address));
            (actual_cpu_cycles % 2) as u8 //FIX: Add APU and IO registers
        } else if requested_address >= PPU_REGS_MIRRORS.start { // PPU REGS
            inst_assert!((PPU_REGS_MIRRORS.start..=PPU_REGS_MIRRORS.end).contains(&requested_address));
            self.ppu.read_from_registers(requested_address % 8) //TODO: Implement this function
        } else if requested_address >= PPU_REGS.start { // PPU REGS
            inst_assert!((PPU_REGS.start..=PPU_REGS.end).contains(&requested_address));
            self.ppu.read_from_registers(requested_address - PPU_REGS.start) //TODO: Implement this function
        } else if requested_address >= RAM_MIRRORS.start { // RAM MIRRORS
            inst_assert!((RAM_MIRRORS.start..=RAM_MIRRORS.end).contains(&requested_address));
            self.memory.ram()[requested_address % RAM.size]
        } else { // RAM
            inst_assert!(requested_address <= RAM.end);
            self.memory.ram()[requested_address]
        }
    }

    pub fn write_8bit_cpu<T>(&mut self, requested_address: T, value: u8, actual_cpu_cycles: &usize)
    where 
        T: Into<usize> + Copy
    {
        let requested_address: usize = requested_address.into();
        inst_assert!(requested_address <= u16::MAX as usize);

        if requested_address > EXPANSION_ROM.start {
            inst_assert!((EXPANSION_ROM.start..=(u16::MAX as usize)).contains(&requested_address));
            self.mapper.write(requested_address, value, self.memory.prg_data_mut());
        } else if requested_address >= APU_REGS.start {
            inst_assert!((APU_REGS.start..=APU_IO_FUNC.end).contains(&requested_address));
            if *actual_cpu_cycles > 0xFFFFFFFF {
                println!("here")
            }
            //FIX: Add APU and IO registers
        } else if requested_address >= PPU_REGS_MIRRORS.start { // PPU REGS
            inst_assert!((PPU_REGS_MIRRORS.start..=PPU_REGS_MIRRORS.end).contains(&requested_address));
            self.ppu.write_to_registers(requested_address % 8, value); //TODO: Implement this function
        } else if requested_address >= PPU_REGS.start { // PPU REGS
            inst_assert!((PPU_REGS.start..=PPU_REGS.end).contains(&requested_address));
            self.ppu.write_to_registers(requested_address - PPU_REGS.start, value); //TODO: Implement this function
        } else if requested_address >= RAM_MIRRORS.start { // RAM MIRRORS
            inst_assert!((RAM_MIRRORS.start..=RAM_MIRRORS.end).contains(&requested_address));
            self.memory.ram_mut()[requested_address % RAM.size] = value
        } else { // RAM
            inst_assert!(requested_address <= RAM.end);
            self.memory.ram_mut()[requested_address] = value;
        }
    }
}

impl Bus {
    pub fn read_8bit_ppu<T>(&mut self, requested_address: T) -> u8
    where 
        T: Into<usize> + Copy
    {
        let requested_address: usize = requested_address.into();
        inst_assert!(requested_address <= 0b0011_1111_1111_1111);

        if requested_address < PPU_NAME_TABLES.start {
            inst_assert!((PPU_PATTERN_TABLES.start..=PPU_PATTERN_TABLES.end).contains(&requested_address)); //TODO: PATTERN TABLES READ
            self.mapper.read_ppu(requested_address, self.memory.chr_data())
        } else if requested_address < PPU_UNUSED_SPACE.start {
            inst_assert!((PPU_NAME_TABLES.start..=PPU_NAME_TABLES.end).contains(&requested_address)); //TODO: NAME_TABLES READ
            self.memory.vram()[requested_address - PPU_NAME_TABLES.start]
        } else if requested_address < PPU_PALETTES.start {
            inst_assert!((PPU_UNUSED_SPACE.start..=PPU_UNUSED_SPACE.end).contains(&requested_address)); //TODO: UNUSED SPACE READ
            warn!("Tried to read from PPU unused space");
            0
        } else {
            inst_assert!((PPU_PALETTES.start..=PPU_PALETTES.end).contains(&requested_address)); //TODO: PPU_PALETTES READ
            self.memory.palettes_table()[requested_address - PPU_PALETTES.start]
        }
    }
}

impl Bus {
    pub fn execute_modules(&mut self) {

    }
}
