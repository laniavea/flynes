use better_assertions::inst_assert;
use log::warn;

use crate::debug_info::{BusDataSource, BusDebugInfo, BusOperationInfo};
use crate::mappers::{MapperRW, Mappers};
use crate::memory::Memory;
use crate::memory::{
    APU_IO_FUNC, APU_REGS, EXPANSION_ROM, PPU_REGS, PPU_REGS_MIRRORS, RAM, RAM_MIRRORS,
};
use crate::memory::{PPU_NAME_TABLES, PPU_PALETTES, PPU_PATTERN_TABLES, PPU_UNUSED_SPACE};
use crate::ppu::Ppu;

#[derive(Debug, Clone, Copy)]
pub enum AddressCpu {
    RAM(usize),
    PpuRegs(usize),
    ApuRegs(usize),
    ROM(usize),
}

#[derive(Debug, Copy, Clone, Default)]
pub enum InterruptStatus {
    #[default]
    None,
    NMI,
}

#[derive(Debug, Clone, Default)]
pub struct Bus {
    memory: Memory,
    ppu: Ppu,
    mapper: Mappers,
    interrupt_status: InterruptStatus,
    debug_info: Option<Vec<BusDebugInfo>>,
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

    pub fn interrupt_status(&mut self) -> InterruptStatus {
        self.interrupt_status
    }

    pub fn set_interrupt_status(&mut self, new_interrupt_status: InterruptStatus) {
        self.interrupt_status = new_interrupt_status;
    }

    pub fn set_debug(&mut self, need_debug: bool) {
        if need_debug {
            if let Some(debug_info) = self.debug_info.as_mut() {
                debug_info.clear();
            } else {
                self.debug_info = Some(vec![])
            }
        } else if self.debug_info.is_some() {
            self.debug_info = None;
        }
    }
}

impl Bus {
    #[inline]
    fn debug_info_read_cpu(
        &mut self,
        actual_cpu_cycles: &usize,
        requested_address: usize,
        data_source: BusDataSource,
        read_value: u8,
    ) {
        if let Some(debug_info) = self.debug_info.as_mut() {
            debug_info.push(BusDebugInfo::new(
                *actual_cpu_cycles,
                requested_address as u16,
                data_source,
                BusOperationInfo::Read(read_value),
            ))
        }
    }

    #[inline]
    fn debug_info_write_cpu(
        &mut self,
        actual_cpu_cycles: &usize,
        requested_address: usize,
        data_source: BusDataSource,
        write_value: u8,
    ) {
        if let Some(debug_info) = self.debug_info.as_mut() {
            debug_info.push(BusDebugInfo::new(
                *actual_cpu_cycles,
                requested_address as u16,
                data_source,
                BusOperationInfo::Write(write_value),
            ))
        }
    }
}

impl Bus {
    pub fn decode_address_from_cpu(address: usize) -> AddressCpu {
        match address {
            address if address > EXPANSION_ROM.start => {
                inst_assert!((EXPANSION_ROM.start..=(u16::MAX as usize)).contains(&address));
                AddressCpu::ROM(address)
            }
            address if address > APU_REGS.start => {
                inst_assert!((APU_REGS.start..=APU_IO_FUNC.end).contains(&address));
                AddressCpu::ApuRegs(address - APU_REGS.start)
            }
            address if address > PPU_REGS.start => {
                inst_assert!((PPU_REGS.start..=PPU_REGS_MIRRORS.end).contains(&address));
                AddressCpu::PpuRegs(address % 8)
            }
            _ => {
                inst_assert!((RAM.start..=RAM_MIRRORS.end).contains(&address));
                AddressCpu::RAM(address % RAM.size)
            }
        }
    }

    pub fn read_8bit_cpu<T>(&mut self, requested_address: T, cpu_cycles: &usize) -> u8
    where
        T: Into<usize> + Copy,
    {
        inst_assert!(requested_address.into() <= u16::MAX as usize);
        let address_cpu: AddressCpu = Bus::decode_address_from_cpu(requested_address.into());

        let read_value = match address_cpu {
            AddressCpu::RAM(address) => self.memory.ram()[address],
            AddressCpu::PpuRegs(address) => {
                self.sync_modules(cpu_cycles);
                self.ppu.read_registers(address) //TODO: Implement this function
            }
            AddressCpu::ApuRegs(address) => {
                (address % 2) as u8 //FIX: Add APU and IO registers. Add debug
            }
            AddressCpu::ROM(address) => self.mapper.read(address, self.memory.prg_data()),
        };

        if self.debug_info.is_some() {
            let data_source: BusDataSource = address_cpu.into();
            self.debug_info_read_cpu(
                cpu_cycles,
                requested_address.into(),
                data_source,
                read_value,
            );
        }

        read_value
    }

    pub fn write_8bit_cpu<T>(&mut self, requested_address: T, value: u8, cpu_cycles: &usize)
    where
        T: Into<usize> + Copy,
    {
        inst_assert!(requested_address.into() <= u16::MAX as usize);
        let address_cpu: AddressCpu = Bus::decode_address_from_cpu(requested_address.into());

        if self.debug_info.is_some() {
            let data_source: BusDataSource = address_cpu.into();
            self.debug_info_write_cpu(cpu_cycles, requested_address.into(), data_source, value);
        }

        match address_cpu {
            AddressCpu::RAM(address) => {
                self.memory.ram_mut()[address] = value;
            }
            AddressCpu::PpuRegs(address) => {
                self.sync_modules(cpu_cycles);
                self.ppu.write_registers(address, value); //TODO: Implement this functio
            }
            AddressCpu::ApuRegs(_address) => {
                //FIX: Add APU and IO registers
            }
            AddressCpu::ROM(address) => {
                self.mapper
                    .write(address, value, self.memory.prg_data_mut());
            }
        };
    }

    pub fn peek_8bit_cpu<T>(&mut self, requested_address: T) -> u8
    where
        T: Into<usize> + Copy,
    {
        let address_cpu: AddressCpu = Bus::decode_address_from_cpu(requested_address.into());

        match address_cpu {
            AddressCpu::RAM(address) => self.memory.ram()[address],
            AddressCpu::PpuRegs(address) => self.ppu.peek_registers(address),
            AddressCpu::ApuRegs(address) => {
                (address % 2) as u8 //FIX: Add APU and IO registers.
            }
            // FIX: Change to peek
            AddressCpu::ROM(address) => self.mapper.read(address, self.memory.prg_data()),
        }
    }
}

impl Bus {
    pub fn read_8bit_ppu<T>(&mut self, requested_address: T) -> u8
    where
        T: Into<usize> + Copy,
    {
        let requested_address: usize = requested_address.into();
        inst_assert!(requested_address <= 0b0011_1111_1111_1111);

        if requested_address < PPU_NAME_TABLES.start {
            inst_assert!(
                (PPU_PATTERN_TABLES.start..=PPU_PATTERN_TABLES.end).contains(&requested_address)
            ); //TODO: PATTERN TABLES READ
            self.mapper
                .read_ppu(requested_address, self.memory.chr_data())
        } else if requested_address < PPU_UNUSED_SPACE.start {
            inst_assert!(
                (PPU_NAME_TABLES.start..=PPU_NAME_TABLES.end).contains(&requested_address)
            ); //TODO: NAME_TABLES READ
            self.memory.vram()[requested_address - PPU_NAME_TABLES.start]
        } else if requested_address < PPU_PALETTES.start {
            inst_assert!(
                (PPU_UNUSED_SPACE.start..=PPU_UNUSED_SPACE.end).contains(&requested_address)
            ); //TODO: UNUSED SPACE READ
            warn!("Tried to read from PPU unused space");
            0
        } else {
            inst_assert!((PPU_PALETTES.start..=PPU_PALETTES.end).contains(&requested_address)); //TODO: PPU_PALETTES READ
            self.memory.palettes_table()[requested_address - PPU_PALETTES.start]
        }
    }
}

impl Bus {
    pub fn sync_modules(&mut self, actual_cpu_cycles: &usize) {
        self.ppu
            .sync_with_cpu(*actual_cpu_cycles, &mut self.interrupt_status);
    }
}
