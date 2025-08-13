use better_assertions::{inst_assert_eq, fast_assert};
use log::{trace, debug, error};

use crate::memory::{Memory, MemoryType};
use instructions::{Operation, CPUInstByte};

mod instructions;

const CARRY_FLAG: usize = 0;
const ZERO_FLAG: usize = 1;
const INTERRUPT_FLAG: usize = 2;
const DECIMAL_FLAG: usize = 3;
const BREAK_FLAG: usize = 4;
const UNUSED_FLAG: usize = 5;
const OVERFLOW_FLAG: usize = 6;
const NEGATIVE_FLAG: usize = 7;

static INSTRUCTION_SET: [Operation; 256] = instructions::init_all_operations().0;
static INSTRUCTION_COUNT: usize = instructions::init_all_operations().1;

#[derive(Debug)]
pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    cpu_status: u8,
    stack_pointer: u8,
    program_counter: u16,
    instruction_set: &'static [Operation; 256],
}

impl Default for Cpu {
    fn default() -> Cpu {
        inst_assert_eq!(
            INSTRUCTION_COUNT,
            INSTRUCTION_SET.iter().filter(|i| !matches!(i.op_name(), CPUInstByte::NoOp)).count()
        );

        Cpu {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            cpu_status: 0b0000_0000,
            stack_pointer: 0xFF,
            program_counter: 0xFFFF,
            instruction_set: &INSTRUCTION_SET,
        }
    }
}

impl Cpu {
    pub fn init_pc(&mut self, memory: &Memory) {
        let exec_pc = memory.get_16bit_value(0xFFFE);
        self.program_counter = exec_pc;
        debug!("Initialized PC: {exec_pc}")
    }

    pub fn init_sp(&mut self, new_stack_pointer: u8) {
        self.stack_pointer = new_stack_pointer
    }

    pub fn stack_pointer_mut(&mut self) -> &mut u8 {
        &mut self.stack_pointer
    }
}

impl Cpu {
    pub fn get_by_1byte_address<'a>(&self, mt: MemoryType, value: &'a mut u8, memory: &'a mut Memory) -> &'a mut u8 {
        fast_assert!([
            MemoryType::Immediate,
            MemoryType::ZeroPage,
            MemoryType::ZeroPageX,
            MemoryType::ZeroPageY,
            MemoryType::Relative,
            MemoryType::IndirectX,
            MemoryType::IndirectY,
        ].contains(&mt));

        match mt {
            MemoryType::Immediate => {
                value
            },
            MemoryType::ZeroPage => {
                memory.get_mut_8bit_value(*value)
            },
            MemoryType::ZeroPageX => {
                let value = value.wrapping_add(self.reg_x);
                memory.get_mut_8bit_value(value)
            },
            MemoryType::ZeroPageY => {
                let value = value.wrapping_add(self.reg_y);
                memory.get_mut_8bit_value(value)
            },
            MemoryType::Relative => {
                value
            },
            MemoryType::IndirectX => {
                let value = value.wrapping_add(self.reg_x);
                let new_address = memory.get_16bit_value(value as u16);
                memory.get_mut_8bit_value(new_address)
            },
            MemoryType::IndirectY => {
                let value = value.wrapping_add(self.reg_y);
                let new_address = memory.get_16bit_value(value as u16);
                memory.get_mut_8bit_value(new_address)
            },
            _ => unreachable!(),
        }
    }

    pub fn conv_2byte_address(&self, mt: MemoryType, value: u16, memory: &Memory) -> u16 {
        fast_assert!([
            MemoryType::Absolute,
            MemoryType::AbsoluteX,
            MemoryType::AbsoluteY,
            MemoryType::Indirect,
        ].contains(&mt));

        match mt {
            MemoryType::Absolute => {
                value
            },
            MemoryType::Indirect => {
                memory.get_16bit_value(value)
            },
            MemoryType::AbsoluteX => {
                value.wrapping_add(self.reg_x as u16)
            },
            MemoryType::AbsoluteY => {
                value.wrapping_add(self.reg_y as u16)
            },
            _ => unreachable!(),
        }
    }
}

impl Cpu {
    pub fn run_cpu(&mut self, memory: &mut Memory) {
        debug!("Running CPU with next PC: {}", self.program_counter);

        let max_number_of_operations = 600_000;
        let mut now_oper: usize = 0;

        while now_oper < max_number_of_operations {
            if self.execute_cpu_iteration(memory).is_err() {
                // break
            }
            now_oper += 1;
        }
    }

    pub fn execute_cpu_iteration(&mut self, memory: &mut Memory) -> Result<u8, &'static str> {
        let now_command = memory.get_8bit_value(self.program_counter);
        let now_inst = self.instruction_set[now_command as usize];
        trace!("CPU got command hex: {now_command}, instruction: {now_inst}");
        trace!("Working with {} bytes of data from {}", now_inst.op_name().as_digit(), self.program_counter);

        match now_inst.op_name() {
            CPUInstByte::One(inst_entry) => {
                self.execute_inst_1_byte(inst_entry, memory);
            },
            CPUInstByte::Two(inst_entry) => {
                let mut next_data_byte = memory.get_8bit_value(self.program_counter.wrapping_add(1));
                let target_byte = self.get_by_1byte_address(now_inst.memory_type(), &mut next_data_byte, memory);
                self.program_counter += 1;
                self.execute_inst_2_byte(inst_entry, target_byte);
            },
            CPUInstByte::Three(inst_entry) => {
                let next_value = memory.get_16bit_value(self.program_counter.wrapping_add(1));
                let target_address = self.conv_2byte_address(now_inst.memory_type(), next_value, memory);
                self.program_counter += 2;
                self.execute_inst_3_byte(inst_entry, target_address, memory);
            },
            CPUInstByte::NoOp => {
                // error!("Trying to parse NoOp instruction at {} with hex {now_command}", self.program_counter);
                return Err("NoOp parsed")
            }
        }
        self.program_counter += 1;

        Ok(now_inst.cycles())
    }
}
