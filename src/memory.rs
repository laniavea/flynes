use better_assertions::{inst_assert, inst_assert_eq};
use log::{debug, warn};

struct MemoryAllocInfo {
    start: usize,
    end: usize,
    size: usize,
    comp_start: usize,
    comp_end: usize,
}

const RAM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x0000,
    end: 0x07FF,
    size: 0x0800,
    comp_start: 0x0000,
    comp_end: 0x07FF,
};

const RAM_MIRRORS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x0800,
    end: 0x1FFF,
    size: 0x1800,
    comp_start: 0x0000,
    comp_end: 0x07FF,
};

const PPU_REGS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x2000,
    end: 0x2007,
    size: 0x0008,
    comp_start: RAM.comp_end+1,
    comp_end: RAM.comp_end+0x0008,
};

const PPU_REGS_MIRRORS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x2008,
    end: 0x3FFF,
    size: 0x1FF8,
    comp_start: RAM.comp_end+1,
    comp_end: RAM.comp_end+0x0008,
};

const APU_REGS: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4000,
    end: 0x4017,
    size: 0x0018,
    comp_start: PPU_REGS.comp_end+1,
    comp_end: PPU_REGS.comp_end+0x0018,
};

const APU_IO_FUNC: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4018,
    end: 0x401F,
    size: 0x0008,
    comp_start: APU_REGS.comp_end+1,
    comp_end: APU_REGS.comp_end+0x0008,
};

const EXPANSION_ROM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x4020,
    end: 0x5FFF,
    size: 0x1FE0,
    comp_start: APU_IO_FUNC.comp_end+1,
    comp_end: APU_IO_FUNC.comp_end+0x1FE0,
};

const SRAM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x6000,
    end: 0x7FFF,
    size: 0x2000,
    comp_start: EXPANSION_ROM.comp_end+1,
    comp_end: EXPANSION_ROM.comp_end+0x2000,
};

const PRG_ROM: MemoryAllocInfo = MemoryAllocInfo {
    start: 0x8000,
    end: 0xFFFF,
    size: 0x8000,
    comp_start: SRAM.comp_end+1,
    comp_end: SRAM.comp_end+0x8000,
};

const ALL_COMP_MEMORY_SIZE: usize = RAM.size + PPU_REGS.size
    + APU_REGS.size + APU_IO_FUNC.size
    + EXPANSION_ROM.size + SRAM.size + PRG_ROM.size;

const STACK_END: usize = 0x0100;
const STACK_START: usize = STACK_END + 0x00FF;

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

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    data: [u8; ALL_COMP_MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Memory {
        inst_assert_eq!(ALL_COMP_MEMORY_SIZE + RAM_MIRRORS.size + PPU_REGS_MIRRORS.size, (u16::MAX as usize) + 1);
        Memory {
            data: [0u8; ALL_COMP_MEMORY_SIZE],
        }
    }
}

impl Memory {
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        debug!("Loading ROM");

        let mut now_index = PRG_ROM.comp_start;
        for now_byte in rom_data {
            inst_assert!(now_index <= PRG_ROM.comp_end);
            self.data[now_index] = *now_byte;
            now_index += 1;
        }

        println!("{:?}", self.data.iter().skip(self.data.len() - 10).collect::<Vec<&u8>>());

        debug!("ROM was loaded with next size: {}", rom_data.len());
    }
}

impl Memory {
    pub fn ram(&self) -> &[u8] {
        inst_assert_eq!(self.data[0..=RAM.end].len(), RAM.size);
        &self.data[0..=RAM.end]
    }

    pub fn ppu(&self) -> &[u8] {
        inst_assert_eq!(self.data[PPU_REGS.comp_start..=PPU_REGS.comp_end].len(), PPU_REGS.size);
        &self.data[PPU_REGS.comp_start..=PPU_REGS.comp_end]
    }

    pub fn apu(&self) -> &[u8] {
        inst_assert_eq!(self.data[APU_REGS.comp_start..=APU_REGS.comp_end].len(), APU_REGS.size);
        &self.data[APU_REGS.comp_start..=APU_REGS.comp_end]
    }

    pub fn apu_io_func(&self) -> &[u8] {
        let res = &self.data[APU_IO_FUNC.comp_start..=APU_IO_FUNC.comp_end];
        inst_assert_eq!(res.len(), APU_IO_FUNC.size);
        res
    }
}

impl Memory {
    pub fn get_8bit_value<T>(&self, requested_address: T) -> u8
    where 
        T: Into<usize> + Copy
    {
        let requested_address = requested_address.into();
        inst_assert!(requested_address <= u16::MAX as usize);

        // APU REGS AND HIGHER
        if requested_address > PPU_REGS_MIRRORS.end {
            inst_assert!(requested_address >= APU_REGS.start);
            self.data[APU_REGS.comp_start + (requested_address - (APU_REGS.start))]

        // PPU REGS AND IT'S MIRRORS
        } else if requested_address > RAM_MIRRORS.end {
            inst_assert!((PPU_REGS.start..=PPU_REGS_MIRRORS.end).contains(&requested_address));
            self.data[PPU_REGS.comp_start + (requested_address % PPU_REGS.size)]

        // RAM MIRRORS
        } else if requested_address > RAM.end {
            inst_assert!((RAM_MIRRORS.start..=RAM_MIRRORS.end).contains(&requested_address));
            self.data[requested_address % RAM.size]

        // RAM
        } else {
            inst_assert!(requested_address <= RAM.end);
            self.data[requested_address]
        }
    }

    pub fn get_mut_8bit_value<T>(&mut self, requested_address: T) -> &mut u8
    where 
        T: Into<usize> + Copy
    {
        let requested_address = requested_address.into();
        inst_assert!(requested_address <= u16::MAX as usize);

        // APU REGS AND HIGHER
        if requested_address > PPU_REGS_MIRRORS.end {
            inst_assert!(requested_address >= APU_REGS.start);
            &mut self.data[APU_REGS.comp_start + (requested_address - (APU_REGS.start))]

        // PPU REGS AND IT'S MIRRORS
        } else if requested_address > RAM_MIRRORS.end {
            inst_assert!((PPU_REGS.start..=PPU_REGS_MIRRORS.end).contains(&requested_address));
            &mut self.data[PPU_REGS.comp_start + (requested_address % PPU_REGS.size)]

        // RAM MIRRORS
        } else if requested_address > RAM.end {
            inst_assert!((RAM_MIRRORS.start..=RAM_MIRRORS.end).contains(&requested_address));
            &mut self.data[requested_address % RAM.size]

        // RAM
        } else {
            inst_assert!(requested_address <= RAM.end);
            &mut self.data[requested_address]
        }
    }

    pub fn get_16bit_value(&self, requested_address: u16) -> u16 {
        let next_address = requested_address.wrapping_add(1) as usize;

        // RAM
        let (next_byte, requested_byte) = if next_address <= RAM.end {
            inst_assert!(next_address <= RAM.end);
            (
                self.data[next_address],
                self.data[next_address.checked_sub(1).unwrap_or(ALL_COMP_MEMORY_SIZE - 1)]
            )

        // RAM MIRRORS
        } else if next_address <= RAM_MIRRORS.end {
            inst_assert!((RAM_MIRRORS.start..=RAM_MIRRORS.end).contains(&next_address));
            let shifted_address = next_address % RAM.size;
            (
                self.data[shifted_address],
                self.data[shifted_address.checked_sub(1).unwrap_or(RAM.end)]
            )

        // PPU REGS AND MIRRORS
        } else if next_address <= PPU_REGS_MIRRORS.end {
            inst_assert!((PPU_REGS.start..=PPU_REGS_MIRRORS.end).contains(&next_address));
            if next_address < PPU_REGS_MIRRORS.start {
                let shifted_address = PPU_REGS.comp_start + (next_address - PPU_REGS.start);
                (
                    self.data[shifted_address],
                    self.data[shifted_address - 1]
                )
            } else {
                let ppu_shift = next_address % PPU_REGS.size;
                (
                    self.data[PPU_REGS.comp_start + ppu_shift],
                    self.data[PPU_REGS.comp_start + ppu_shift.checked_sub(1).unwrap_or(PPU_REGS.size - 1)]
                )
            }

        // APU REGS AND HIGHER
        } else {
            inst_assert!(next_address >= APU_REGS.start);
            let shifted_address = APU_REGS.comp_start + (next_address - APU_REGS.start);
            (
                self.data[shifted_address],
                self.data[shifted_address - 1]
            )
        };

        ((next_byte as u16) << 8) + requested_byte as u16
    }
}

impl Memory {
    pub fn stack_push_8bit(&mut self, value: u8, stack_pointer: &mut u8) {
        self.data[STACK_END + (*stack_pointer as usize)] = value;
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
        self.data[STACK_END + (*stack_pointer as usize)]
    }

    pub fn stack_push_16bit(&mut self, value: u16, stack_pointer: &mut u8) {
        let first_addr = STACK_END + (*stack_pointer as usize);
        self.data[first_addr] = (value >> 8) as u8;

        let second_addr = STACK_END + (stack_pointer.wrapping_sub(1) as usize);
        self.data[second_addr] = value as u8;

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

        (self.data[first_addr] as u16) + ((self.data[second_addr] as u16) << 8)
    }

    pub fn stack_as_slice(&self) -> &[u8] {
        let stack_copy:&[u8] = &self.data[STACK_END..=STACK_START];
        inst_assert_eq!(stack_copy.len(), 256);
        stack_copy
    }
}

#[test]
fn test_read_write_cpu_mem() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let mut mem = Memory::default();

    *mem.get_mut_8bit_value(0x0800u16) = 0x42;
    *mem.get_mut_8bit_value(0x1001u16) = 0x43;
    assert_eq!((mem.get_8bit_value(0x0000u16), mem.get_8bit_value(0x1801u16)), (0x42, 0x43));

    *mem.get_mut_8bit_value(0x07FEu16) = 0x19;
    *mem.get_mut_8bit_value(0x1FFFu16) = 0x20;
    assert_eq!((mem.get_8bit_value(0x0FFEu16), mem.get_8bit_value(0x17FFu16)), (0x19, 0x20));

    for now_i in 0..=7 {
        *mem.get_mut_8bit_value(0x2000u16 + now_i) = now_i as u8 + 1;
    }

    for now_i in 0..=7 {
        assert_eq!(mem.get_8bit_value(0x3FF8u16 + now_i), now_i as u8 + 1);
    }

    for now_i in 0..=7 {
        *mem.get_mut_8bit_value(0x2008u16 + now_i) = now_i as u8 + 100;
    }

    for now_i in 0..=7 {
        assert_eq!(mem.get_8bit_value(0x2000u16 + now_i), now_i as u8 + 100);
    }

    for i in 0..100_000 {
        // Tests for 8bit read/write
        let act_pointer = rng.random::<u16>();
        let random_v = rng.random::<u8>();
        let random_ram_mod = (((rng.random::<u8>() % 4) as usize) * 0x0800) as u16;

        *mem.get_mut_8bit_value(act_pointer) = random_v;

        if act_pointer <= 0x1FFF {
            let read_val = mem.get_8bit_value((act_pointer % 0x0800) + random_ram_mod);
            assert_eq!(read_val, random_v)
        } else {
            assert_eq!(mem.get_8bit_value(act_pointer), random_v)
        }

        // Tests for 16bit read/write
        let upper_bit = rng.random::<u8>() as u16;
        let lower_bit = rng.random::<u8>() as u16;
        let now_data = (upper_bit << 8) + lower_bit;

        *mem.get_mut_8bit_value(act_pointer) = lower_bit as u8;
        *mem.get_mut_8bit_value(act_pointer.wrapping_add(1)) = upper_bit as u8;

        let read_data16b = mem.get_16bit_value(act_pointer);
        assert_eq!(now_data, read_data16b, "Exptected {now_data}, got {read_data16b}, at {act_pointer} <- i {i})");
    }
}

#[test]
fn test_stack_push_pull() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    use crate::cpu::Cpu;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    let mut mem: Memory = Memory {
        data: [0u8; ALL_COMP_MEMORY_SIZE],
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
