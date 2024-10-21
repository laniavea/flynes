use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
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

const BASE_STACK_POINTER: u16 = 0x01FF;

// Memory Structure:
// $0000 - $00FF -> ZeroPage (can be accessed with fewer bytes and cycles than other modules)
// $0100 - $01FF -> Stack (can be start anywhere but usually starts from 01FF and grows downward)
// $0200 - $07FF -> RAM
// $0800 - $1FFF -> Mirrors for $0000 - $07FF
// $2000 - $2007 -> IO registers
// $2008 - $3FFF -> Mirrors for $2000 - $2007
// $4000 - $401F -> IO registers
// $4020 - $5FFF -> Expansion ROM
// $6000 - $7FFF -> SRAM
// $8000 - $BFFF -> PRG-ROM LB
// $C000 - $FFFF -> PRG-ROM UB
// More in https://www.nesdev.org/NESDoc.pdf

impl Cpu {
    /// Function to read block of data from memory (0x0000 to 0xFFFF)
    pub fn read_mem(&self, pointer: u16) -> u8 {
        if pointer < 0x2000 {
            self.memory[(pointer % 0x0800) as usize]
        } else if (0x4000..=0x5FFF).contains(&pointer) {
            self.memory[(0x4000 + (pointer % 8)) as usize]
        } else {
            self.memory[pointer as usize]
        }
    }

    /// Function to write block of data to memory (0x0000 to 0xFFFF)
    pub fn write_mem(&mut self, pointer: u16, data: u8) {
        if pointer < 0x2000 {
            self.memory[(pointer % 0x0800) as usize] = data;
        } else if (0x4000..=0x5FFF).contains(&pointer) {
            self.memory[(0x4000 + (pointer % 8)) as usize] = data;
        } else {
            self.memory[pointer as usize] = data;
        }
    }

    /// Function to read block and next block of data from memory (0x0000 to 0xFFFF)
    /// First block is lower bits of a result, for example (05 01) will be transformed to 0x0105
    /// Little-endian Byte Order
    pub fn read_mem_16b(&self, pointer: u16) -> u16 {
        if pointer < 0x2000 {
            let act_pointer = pointer % 0x0800;
            let next_pointer = if pointer != 0x1FFF { (pointer + 1) % 0x0800 } else { 0x2000 };
            (self.memory[act_pointer as usize] as u16).wrapping_add((self.memory[next_pointer as usize] as u16) << 8)

        } else if (0x4000..=0x5FFF).contains(&pointer) {
            let act_pointer = 0x4000 + (pointer % 8);
            let next_pointer = if pointer != 0x5FFF { 0x4000 + ((act_pointer + 1) % 8) } else { 0x6000 };

            (self.memory[act_pointer as usize] as u16).wrapping_add((self.memory[next_pointer as usize] as u16) << 8)

        } else {
            (self.memory[pointer as usize] as u16).wrapping_add((self.memory[pointer.wrapping_add(1) as usize] as u16) << 8)
        }
    }

    /// Function to write block and next block of data from memory (0x0000 to 0xFFFF)
    /// First block is lower bits of a result, for example (05 01) will be transformed to 0x0105
    /// Little-endian Byte Order
    pub fn write_mem_16b(&mut self, pointer: u16, data: u16) {
        let second_byte = data as u8;
        let first_byte = (data >> 8) as u8;

        if pointer < 0x2000 {
            let act_pointer = pointer % 0x0800;
            let next_pointer = if pointer != 0x1FFF { (pointer + 1) % 0x0800 } else { 0x2000 };
            self.memory[act_pointer as usize] = second_byte;
            self.memory[next_pointer as usize] = first_byte;

        } else if (0x4000..=0x5FFF).contains(&pointer) {
            let act_pointer = 0x4000 + (pointer % 8);
            let next_pointer = if pointer != 0x5FFF { 0x4000 + ((act_pointer + 1) % 8) } else { 0x6000 };
            self.memory[act_pointer as usize] = second_byte;
            self.memory[next_pointer as usize] = first_byte;

        } else {
            self.memory[pointer as usize] = second_byte;
            self.memory[pointer.wrapping_add(1) as usize] = first_byte;
        }
    }

    /// Function to pop data from stack by stack pointer
    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.read_mem(BASE_STACK_POINTER - (self.stack_pointer as u16))
    }

    /// Function to push data to stack by stack pointer
    pub fn stack_push(&mut self, value: u8) {
        self.write_mem(BASE_STACK_POINTER - self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
    }

    /// Function to pop data (2x 8bit vals in Little-endian order) from stack 
    pub fn stack_pop_16b(&mut self) -> u16 {
        (self.stack_pop() as u16) + ((self.stack_pop() as u16) << 8)
    }
}

impl Cpu {
    /// Function to read data from memory in different modes from selected pointer.
    /// The meaning of pointer changes based on memory_type.
    /// more info at https://www.nesdev.org/obelisk-6502-guide/addressing.html
    pub fn ref_to_memory_by_address(&self, pointer: u16, memory_type: MemoryType) -> u16 {
        match memory_type {
            MemoryType::Accumulator => {
                0
            },
            MemoryType::Immediate => {
                pointer
            },
            MemoryType::ZeroPage => {
                pointer
            },
            MemoryType::ZeroPageX => {
                (pointer as u8 + self.reg_x) as u16
            },
            MemoryType::ZeroPageY => {
                (pointer as u8 + self.reg_y) as u16
            },
            MemoryType::Absolute => {
                pointer
            },
            MemoryType::AbsoluteX => {
                pointer + self.reg_x as u16
            },
            MemoryType::AbsoluteY => {
                pointer + self.reg_y as u16
            },
            MemoryType::Indirect => {
                self.read_mem_16b(pointer)
            },
            MemoryType::IndirectX => {
                let zero_page_add = (self.reg_x + pointer as u8) as usize;
                self.read_mem_16b(zero_page_add as u16)
            },
            MemoryType::IndirectY => {
                let zero_page_add = (self.reg_y + pointer as u8) as usize;
                self.read_mem_16b(zero_page_add as u16)
            },
            _ => {
                unimplemented!();
            },
        }
    }
}

#[test]
fn test_read_write_cpu_mem() {
    let mut cpu = Cpu::default();

    cpu.write_mem(0x0800, 0x42);
    cpu.write_mem(0x1001, 0x43);
    assert_eq!((cpu.read_mem(0x0000), cpu.read_mem(0x1801)), (0x42, 0x43));

    cpu.write_mem(0x07FE, 0x19);
    cpu.write_mem(0x1FFF, 0x20);
    assert_eq!((cpu.read_mem(0x0FFE), cpu.read_mem(0x17FF)), (0x19, 0x20));

    for now_i in 0..=7 {
        cpu.write_mem(0x4000 + now_i, now_i as u8 + 1);
    }
    for now_i in 0..=7 {
        assert_eq!(cpu.read_mem(0x5FF8 + now_i), now_i as u8 + 1)
    }

    for now_i in 0..=7 {
        cpu.write_mem(0x4008 + now_i, now_i as u8 + 1);
    }
    for now_i in 0..=7 {
        assert_eq!(cpu.read_mem(0x4000 + now_i), now_i as u8 + 1)
    }

    let mut seed: u32 = 52;
    for _ in 0..1000000 {
        // PRNG
        let mut num = seed;
        num ^= num << 13;
        num ^= num >> 17;
        num ^= num << 5;
        seed = num;

        // Tests for 8bit read/write
        let act_pointer = (seed % (u16::MAX as u32)) as u16;
        cpu.write_mem(act_pointer, (act_pointer % 256) as u8);

        if act_pointer <= 0x1FFF {
            let read_val = cpu.read_mem((act_pointer % 0x0800) + ((act_pointer % 4) * 0x0800));
            assert_eq!(read_val, (act_pointer % 256) as u8)
        } else {
            assert_eq!(cpu.read_mem(act_pointer), (act_pointer % 256) as u8)
        }

        // Tests for 16bit read/write
        let first_data = act_pointer / 256;
        let second_data = act_pointer % 256;
        let now_data = (first_data << 8) + second_data;

        cpu.write_mem(act_pointer, second_data as u8);
        cpu.write_mem(act_pointer.wrapping_add(1), first_data as u8);

        let readed_data_16_b = cpu.read_mem_16b(act_pointer);
        assert_eq!(now_data, readed_data_16_b);

        cpu.write_mem_16b(act_pointer, readed_data_16_b);
        let readed_data_8b = (cpu.read_mem(act_pointer) as u16).wrapping_add((cpu.read_mem(act_pointer.wrapping_add(1)) as u16) << 8);
        assert_eq!(readed_data_16_b, readed_data_8b);
    }
}

#[test]
fn test_stack_pop_push() {
    let basic_stack_pointer: u8 = 0x10;
    let mut cpu = Cpu {
        stack_pointer: basic_stack_pointer,
        ..Default::default()
    };

    let mut stack_ideal = [0u8; 256];
    for now_i in 0..=0xFF {
        cpu.stack_push(now_i);
        stack_ideal[0xFF - now_i as usize] = now_i;
    }
    // Rotate left because stack is reversed and grows downward
    stack_ideal.rotate_left(cpu.stack_pointer as usize);

    assert_eq!(stack_ideal, cpu.memory[0x100..0x200]);

    assert_eq!(cpu.stack_pop(), 0xFF);
    assert_eq!(cpu.stack_pop_16b(), (0xFE + (0xFD<<8)));
    assert_eq!(cpu.stack_pointer, basic_stack_pointer - 3);

    for _ in 0..cpu.stack_pointer {
        cpu.stack_pop();
    }

    assert_eq!(cpu.stack_pointer, 0x00);
    assert_eq!(cpu.stack_pop_16b(), (0xFF - basic_stack_pointer as u16) + ((0xFF - basic_stack_pointer as u16 - 1) << 8));
}
