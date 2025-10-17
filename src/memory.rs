use better_assertions::inst_assert_eq;
use log::warn;

pub struct MemoryAllocInfo {
    pub start: usize,
    pub end: usize,
    pub size: usize,
}

pub const RAM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x0000,
    end: 0x07FF,
    size: 0x0800,
};

pub const RAM_MIRRORS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x0800,
    end: 0x1FFF,
    size: 0x1800,
};

pub const PPU_REGS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x2000,
    end: 0x2007,
    size: 0x0008,
};

pub const PPU_REGS_MIRRORS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x2008,
    end: 0x3FFF,
    size: 0x1FF8,
};

pub const APU_REGS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4000,
    end: 0x4017,
    size: 0x0018,
};

pub const APU_IO_FUNC: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4018,
    end: 0x401F,
    size: 0x0008,
};

pub const EXPANSION_ROM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4020,
    end: 0x5FFF,
    size: 0x1FE0,
};

pub const SRAM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x6000,
    end: 0x7FFF,
    size: 0x2000,
};

pub const PRG_ROM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x8000,
    end: 0xFFFF,
    size: 0x8000,
};

const ALL_COMP_MEMORY_SIZE: usize = RAM.size + PPU_REGS.size
    + APU_REGS.size + APU_IO_FUNC.size
    + EXPANSION_ROM.size + SRAM.size + PRG_ROM.size;

const STACK_END: usize = 0x0100;
const STACK_START: usize = STACK_END + 0x00FF;

pub const PPU_PATTERN_TABLES: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x0000,
    end: 0x1FFF,
    size: 0x2000,
};

pub const PPU_NAME_TABLES: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x2000,
    end: 0x2FFF,
    size: 0x0FFF,
};

pub const PPU_UNUSED_SPACE: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x3000,
    end: 0x3EFF,
    size: 0x0F00
};

pub const PPU_PALETTES: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x3F00,
    end: 0x3F1F,
    size: 0x0020,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MemoryType::Implied => write!(f, "Implied"),
            MemoryType::Accumulator => write!(f, "Accumulator"),
            MemoryType::Immediate => write!(f, "Immediate"),
            MemoryType::ZeroPage => write!(f, "Zero Page"),
            MemoryType::ZeroPageX => write!(f, "Zero Page X"),
            MemoryType::ZeroPageY => write!(f, "Zero Page Y"),
            MemoryType::Relative => write!(f, "Relative"),
            MemoryType::Absolute => write!(f, "Absolute"),
            MemoryType::AbsoluteX => write!(f, "Absolute X"),
            MemoryType::AbsoluteY => write!(f, "Absolute Y"),
            MemoryType::Indirect => write!(f, "Indirect"),
            MemoryType::IndirectX => write!(f, "Indirect X"),
            MemoryType::IndirectY => write!(f, "Indirect Y"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    prg_data: Vec<u8>,
    chr_data: Vec<u8>,
    ram: [u8; RAM.size],
    vram: [u8; PPU_NAME_TABLES.size],
    palettes_table: [u8; PPU_PALETTES.size]
}

impl Default for Memory {
    fn default() -> Memory {
        inst_assert_eq!(ALL_COMP_MEMORY_SIZE + RAM_MIRRORS.size + PPU_REGS_MIRRORS.size, (u16::MAX as usize) + 1);
        Memory {
            prg_data: Vec::new(),
            chr_data: Vec::new(),
            ram: [0u8; RAM.size],
            vram: [0u8; PPU_NAME_TABLES.size],
            palettes_table: [0u8; PPU_PALETTES.size],
        }
    }
}

impl Memory {
    pub fn set_prg_data(&mut self, prg_data: Vec<u8>) {
        self.prg_data = prg_data;
    }
}

impl Memory {
    pub fn prg_data(&self) -> &Vec<u8> {
        &self.prg_data
    }

    pub fn prg_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.prg_data
    }

    pub fn chr_data(&self) -> &Vec<u8> {
        &self.chr_data
    }

    pub fn chr_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.chr_data
    }

    pub fn ram(&self) -> &[u8; RAM.size] {
        &self.ram
    }

    pub fn ram_mut(&mut self) -> &mut[u8; RAM.size] {
        &mut self.ram
    }

    pub fn vram(&self) -> &[u8; PPU_NAME_TABLES.size] {
        &self.vram
    }
    
    pub fn palettes_table(&self) -> &[u8; PPU_PALETTES.size] {
        &self.palettes_table
    }
}

impl Memory {
    pub fn write_prg_rom(&mut self, prg_rom: &[u8]) {
        self.prg_data = prg_rom.to_vec();
        // let mut now_byte_id = PRG_ROM.comp_start;
        // for now_byte in prg_rom {
        //     self.cpu_data[now_byte_id] = *now_byte;
        //     now_byte_id += 1;
        // }
    }
}

impl Memory {
    pub fn stack_push_8bit(&mut self, value: u8, stack_pointer: &mut u8) {
        self.ram[STACK_END + (*stack_pointer as usize)] = value;
        *stack_pointer = stack_pointer.wrapping_sub(1);
        if *stack_pointer == u8::MAX {
            warn!("Stack overflow occured after 8bit push");
        }
    }

    pub fn stack_pull_8bit(&self, stack_pointer: &mut u8) -> u8 {
        *stack_pointer = stack_pointer.wrapping_add(1);
        if *stack_pointer == u8::MIN {
            warn!("Stack underflow occured after 8bit pull");
        }
        self.ram[STACK_END + (*stack_pointer as usize)]
    }

    pub fn stack_push_16bit(&mut self, value: u16, stack_pointer: &mut u8) {
        let first_addr = STACK_END + (*stack_pointer as usize);
        self.ram[first_addr] = (value >> 8) as u8;

        let second_addr = STACK_END + (stack_pointer.wrapping_sub(1) as usize);
        self.ram[second_addr] = value as u8;

        *stack_pointer = stack_pointer.wrapping_sub(2); 

        if *stack_pointer >= 0xFE { // True if value was wrapped, because sub to stack_pointer already occured 
            warn!("Stack overflow occured after 16bit PUSH");
        }
    }

    pub fn stack_pull_16bit(&self, stack_pointer: &mut u8) -> u16 {
        let first_addr = STACK_END + (stack_pointer.wrapping_add(1) as usize);
        *stack_pointer = stack_pointer.wrapping_add(2);
        let second_addr = STACK_END + (*stack_pointer as usize);

        if *stack_pointer <= 0x01 { // True if value was wrapped, because add to stack_pointer already occured 
            warn!("Stack underflow occured after 16bit PULL");
        }

        (self.ram[first_addr] as u16) + ((self.ram[second_addr] as u16) << 8)
    }

    pub fn stack_as_slice(&self) -> &[u8] {
        let stack_copy:&[u8] = &self.ram[STACK_END..=STACK_START];
        inst_assert_eq!(stack_copy.len(), 256);
        stack_copy
    }
}

// #[test]
// fn test_read_write_cpu_mem() {
//     use rand::{SeedableRng, Rng};
//     use rand::rngs::StdRng;
//
//     let mut rng: StdRng = StdRng::seed_from_u64(42);
//     let mut mem = Memory::default();
//
//     *mem.get_mut_8bit_value(0x0800u16) = 0x42;
//     *mem.get_mut_8bit_value(0x1001u16) = 0x43;
//     assert_eq!((mem.get_8bit_value(0x0000u16), mem.get_8bit_value(0x1801u16)), (0x42, 0x43));
//
//     *mem.get_mut_8bit_value(0x07FEu16) = 0x19;
//     *mem.get_mut_8bit_value(0x1FFFu16) = 0x20;
//     assert_eq!((mem.get_8bit_value(0x0FFEu16), mem.get_8bit_value(0x17FFu16)), (0x19, 0x20));
//
//     for now_i in 0..=7 {
//         *mem.get_mut_8bit_value(0x2000u16 + now_i) = now_i as u8 + 1;
//     }
//
//     for now_i in 0..=7 {
//         assert_eq!(mem.get_8bit_value(0x3FF8u16 + now_i), now_i as u8 + 1);
//     }
//
//     for now_i in 0..=7 {
//         *mem.get_mut_8bit_value(0x2008u16 + now_i) = now_i as u8 + 100;
//     }
//
//     for now_i in 0..=7 {
//         assert_eq!(mem.get_8bit_value(0x2000u16 + now_i), now_i as u8 + 100);
//     }
//
//     for i in 0..100_000 {
//         // Tests for 8bit read/write
//         let act_pointer = rng.random::<u16>();
//         let random_v = rng.random::<u8>();
//         let random_ram_mod = (((rng.random::<u8>() % 4) as usize) * 0x0800) as u16;
//
//         *mem.get_mut_8bit_value(act_pointer) = random_v;
//
//         if act_pointer <= 0x1FFF {
//             let read_val = mem.get_8bit_value((act_pointer % 0x0800) + random_ram_mod);
//             assert_eq!(read_val, random_v)
//         } else {
//             assert_eq!(mem.get_8bit_value(act_pointer), random_v)
//         }
//
//         // Tests for 16bit read/write
//         let upper_bit = rng.random::<u8>() as u16;
//         let lower_bit = rng.random::<u8>() as u16;
//         let now_data = (upper_bit << 8) + lower_bit;
//
//         *mem.get_mut_8bit_value(act_pointer) = lower_bit as u8;
//         *mem.get_mut_8bit_value(act_pointer.wrapping_add(1)) = upper_bit as u8;
//
//         let read_data16b = mem.get_16bit_value(act_pointer);
//         assert_eq!(now_data, read_data16b, "Exptected {now_data}, got {read_data16b}, at {act_pointer} <- i {i})");
//     }
// }

#[test]
fn test_stack_push_pull() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::Cpu;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut mem: Memory = Memory {
        prg_data: Vec::new(),
        chr_data: Vec::new(),
        ram: [0u8; RAM.size],
        vram: [0u8; PPU_NAME_TABLES.size],
        palettes_table: [0u8; PPU_PALETTES.size],
    };

    let mut cpu: Cpu = Cpu::default();
    cpu.init_sp(0xFF);

    for _ in 0..1000 {
        let random_num = rng.random::<u8>();

        for _ in 0..random_num {
            let random_start = rng.random::<u8>().max(0x2);
            let sp = cpu.stack_pointer_mut();
            *sp = random_start;

            let random_data = rng.random::<u16>();

            let random_data_1bit = (random_data >> 8) as u8;
            let random_data_2bit = random_data as u8;

            assert_eq!(((random_data_1bit as u16) << 8) + (random_data_2bit as u16), random_data);

            mem.stack_push_8bit(random_data_1bit, sp);
            assert_eq!(*sp, random_start - 1);
            mem.stack_push_8bit(random_data_2bit, sp);
            assert_eq!(*sp, random_start - 2);

            let pulled_data = mem.stack_pull_16bit(sp);
            assert_eq!(pulled_data, random_data);
            assert_eq!(*sp, random_start);

            mem.stack_push_16bit(random_data, sp);
            assert_eq!(*sp, random_start - 2);
            let pulled_data = mem.stack_pull_16bit(sp);
            assert_eq!(pulled_data, random_data);

            mem.stack_push_16bit(random_data, sp);
            let pulled_data_1bit = mem.stack_pull_8bit(sp) as u16;
            assert_eq!(*sp, random_start - 1);
            let pulled_data_2bit = mem.stack_pull_8bit(sp) as u16;
            assert_eq!(*sp, random_start);

            assert_eq!(pulled_data_1bit + (pulled_data_2bit << 8), random_data);
        }
    }
}
