use better_assertions::inst_assert;

use crate::memory::Memory;
use crate::memory::{PPU_REGS_MIRRORS, APU_REGS, APU_IO_FUNC, PPU_REGS, RAM_MIRRORS, RAM, EXPANSION_ROM};
use crate::ppu::Ppu;
use crate::mappers::{Mappers, MapperRW};

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory,
    ppu: Ppu,
    mapper: Mappers,
}

impl Bus {
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn set_mapper_and_prgdata(&mut self, mapper: Mappers, prg_data: Vec<u8>) {
        self.mapper = mapper;
        self.memory.set_prg_data(prg_data);
    }
}

impl Bus {
    pub fn read_8bit_cpu<T>(&self, requested_address: T) -> u8
    where 
        T: Into<usize> + Copy
    {
        let requested_address: usize = requested_address.into();

        if requested_address > EXPANSION_ROM.start {
            inst_assert!((EXPANSION_ROM.start..=(u16::MAX as usize)).contains(&requested_address));
            self.mapper.mapper_read(requested_address, self.memory.prg_data())
        } else if requested_address >= APU_REGS.start {
            inst_assert!((APU_REGS.start..=APU_IO_FUNC.end).contains(&requested_address));
            0 //FIX: Add APU and IO registers
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

    pub fn write_8bit_cpu<T>(&mut self, requested_address: T, value: u8)
    where 
        T: Into<usize> + Copy
    {
        let requested_address: usize = requested_address.into();
        inst_assert!(requested_address <= u16::MAX as usize);

        if requested_address > EXPANSION_ROM.start {
            inst_assert!((EXPANSION_ROM.start..=(u16::MAX as usize)).contains(&requested_address));
            self.mapper.mapper_write(requested_address, value, self.memory.prg_data_mut());
        } else if requested_address >= APU_REGS.start {
            inst_assert!((APU_REGS.start..=APU_IO_FUNC.end).contains(&requested_address));
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

    pub fn read_16bit_cpu(&self, requested_address: u16) -> u16 {
        let requested_byte = self.read_8bit_cpu(requested_address);
        let next_byte = self.read_8bit_cpu(requested_address.wrapping_add(1));

        ((next_byte as u16) << 8) + (requested_byte as u16)
    }

    pub fn read_16bit_cpu_zp_wrap(&self, requested_address: u16) -> u16 {
        if requested_address == 0x00FF {
            let first_byte = self.read_8bit_cpu(0x0000usize) as u16;
            let second_byte = self.read_8bit_cpu(0x00FFusize) as u16;
            return (first_byte << 8) + second_byte;
        }

        self.read_16bit_cpu(requested_address)
    }

    pub fn read_16bit_cpu_jmp_bug(&self, requested_address: u16) -> u16 {
        if requested_address & 0x00FF == 0x00FF {
            let first_byte = self.read_8bit_cpu(requested_address & 0xFF00) as u16;
            let second_byte = self.read_8bit_cpu(requested_address) as u16;
            return (first_byte << 8) + second_byte;
        }

        self.read_16bit_cpu(requested_address)
    }
}
