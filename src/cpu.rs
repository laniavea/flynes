use better_assertions::{inst_assert_eq, fast_assert};
use log::{trace, debug, info, warn, error};

use crate::memory::MemoryType;
use crate::bus::Bus;
use crate::common;
use instructions::{Operation, CPUInstByte};

const RESET_ON_CPU_EXEC_ERR: bool = true;

pub mod instructions;

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

#[derive(Debug, Clone, Copy)]
pub enum CpuState {
    Running,
    Stopped,
}

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    cpu_status: u8,
    stack_pointer: u8,
    program_counter: u16,
    instruction_set: &'static [Operation; 256],
    state: CpuState,
    exec_cycles: usize,
}

impl Default for Cpu {
    fn default() -> Cpu {
        inst_assert_eq!(
            INSTRUCTION_COUNT,
            INSTRUCTION_SET.iter().filter(|i| !((i.cycles() == 0) && matches!(i.op_name(), CPUInstByte::NoOp))).count()
        );

        Cpu {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            cpu_status: 0b0010_0100,
            stack_pointer: 0xFD,
            program_counter: 0xFFFF,
            instruction_set: &INSTRUCTION_SET,
            state: CpuState::Running,
            exec_cycles: 0,
        }
    }
}

impl Cpu {
    pub fn init_pc(&mut self, bus: &mut Bus) {
        let exec_pc = self.read_16bit(bus, 0xFFFC);
        self.program_counter = exec_pc;
        debug!("Initialized PC: {}", common::number_to_hex(exec_pc, true))
    }

    pub fn set_pc(&mut self, exec_pc: u16) {
        self.program_counter = exec_pc;
        debug!("Initialized PC: {}", common::number_to_hex(exec_pc, true))
    }

    pub fn init_sp(&mut self, new_stack_pointer: u8) {
        self.stack_pointer = new_stack_pointer
    }

    pub fn stack_pointer_mut(&mut self) -> &mut u8 {
        &mut self.stack_pointer
    }
    
    pub fn get_registers_state(&self) -> (u8, u8, u8) {
        (self.reg_a, self.reg_x, self.reg_y)
    }

    pub fn get_cpu_status(&self) -> u8 {
        self.cpu_status
    }

    pub fn get_stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }
}

impl Cpu {
    pub fn conv_1byte_address(&mut self, mt: MemoryType, value: u8, bus: &mut Bus) -> u16 {
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
                self.program_counter
            },
            MemoryType::ZeroPage => {
                value as u16
            },
            MemoryType::ZeroPageX => {
                let value = value.wrapping_add(self.reg_x);
                value as u16
            },
            MemoryType::ZeroPageY => {
                let value = value.wrapping_add(self.reg_y);
                value as u16
            },
            MemoryType::Relative => {
                self.program_counter
            },
            MemoryType::IndirectX => {
                let value = value.wrapping_add(self.reg_x);
                self.read_16bit_zp_wrap(bus, value as u16)
            },
            MemoryType::IndirectY => {
                let value_data = self.read_16bit_zp_wrap(bus, value as u16);
                value_data.wrapping_add(self.reg_y as u16)
            },
            _ => unreachable!(),
        }
    }

    pub fn conv_2byte_address(&mut self, mt: MemoryType, value: u16, bus: &mut Bus) -> u16 {
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
                self.read_16bit_jmp_bug(bus, value)
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
    #[inline(always)]
    pub fn read_8bit<T>(&mut self, bus: &mut Bus, data_ref: T) -> u8
    where 
        T: Into<usize> + Copy
    {
        bus.read_8bit_cpu(data_ref, &self.exec_cycles)
    }

    #[inline(always)]
    pub fn write_8bit<T>(&mut self, bus: &mut Bus, data_ref: T, data_value: u8)
    where 
        T: Into<usize> + Copy
    {
        bus.write_8bit_cpu(data_ref, data_value, &self.exec_cycles);
    }

    pub fn read_16bit(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        let requested_byte = self.read_8bit(bus, requested_address);
        let next_byte = self.read_8bit(bus, requested_address.wrapping_add(1));

        ((next_byte as u16) << 8) + (requested_byte as u16)
    }

    pub fn read_16bit_zp_wrap(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        if requested_address == 0x00FF {
            let first_byte = self.read_8bit(bus, 0x0000usize) as u16;
            let second_byte = self.read_8bit(bus, 0x00FFusize) as u16;
            return (first_byte << 8) + second_byte;
        }

        self.read_16bit(bus, requested_address)
    }

    pub fn read_16bit_jmp_bug(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        if requested_address & 0x00FF == 0x00FF {
            let first_byte = self.read_8bit(bus, requested_address & 0xFF00) as u16;
            let second_byte = self.read_8bit(bus, requested_address) as u16;
            return (first_byte << 8) + second_byte;
        }

        self.read_16bit(bus, requested_address)
    }
}

impl Cpu {
    pub fn run_cpu(&mut self, bus: &mut Bus) {
        debug!("Running CPU with next PC: {}", common::number_to_hex(self.program_counter, true));

        let max_number_of_operations = 100_000_000;
        let mut now_oper: usize = 0;

        while now_oper < max_number_of_operations {
            match self.execute_cpu_iteration(bus) {
                Ok(_) => now_oper += 1,
                Err(err_msg) => {
                    if RESET_ON_CPU_EXEC_ERR {
                        warn!("RESET ON ERROR: {err_msg}");
                        self.set_pc(0xC000);
                    } else {
                        error!("Error while CPU execution: {err_msg}");
                        break
                    }
                }
            }
        }

        info!("Leaving RUN CPU on {now_oper}");
    }

    /// CHANGE ALSO execute_cpu_iteration_info
    pub fn execute_cpu_iteration(&mut self, bus: &mut Bus) -> Result<u8, &'static str> {
        let now_command = self.read_8bit(bus, self.program_counter);
        let now_inst = self.instruction_set[now_command as usize];
        trace!("CPU got command: {}, instruction: {now_inst}", common::number_to_hex(now_command, true));
        trace!(
            "Working with {} bytes of data from {}",
            now_inst.op_name().as_digit(),
            common::number_to_hex(self.program_counter, true)
        );

        match now_inst.op_name() {
            CPUInstByte::One(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_1_byte(inst_entry, bus);

                if matches!(self.state, CpuState::Stopped) {
                    return Err("CPU was stopped by STP instruction")
                }
            },
            CPUInstByte::Two(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let next_data_byte = self.read_8bit(bus, self.program_counter);
                let target_byte = self.conv_1byte_address(now_inst.memory_type(), next_data_byte, bus);
                trace!("Current data value: {}", common::number_to_hex(target_byte, true));
                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_2_byte(bus, inst_entry, target_byte);
            },
            CPUInstByte::Three(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let next_value = self.read_16bit(bus, self.program_counter);
                let target_address = self.conv_2byte_address(now_inst.memory_type(), next_value, bus);
                trace!("Address:{} -> {}",common::number_to_hex(next_value, true), common::number_to_hex(target_address, true));
                self.program_counter = self.program_counter.wrapping_add(2);
                self.execute_inst_3_byte(bus, inst_entry, target_address);
            },
            CPUInstByte::NoOp => {
                error!(
                    "Trying to parse NoOp instruction at {} with hex {}",
                    common::number_to_hex(self.program_counter, true),
                    common::number_to_hex(now_command, true)
                );
                return Err("NoOp parsed")
            }
        }

        Ok(now_inst.cycles())
    }

    /// CHANGE ALSO execute_cpu_iteration
    pub fn execute_cpu_iteration_info(&mut self, bus: &mut Bus) -> Result<(Operation, Vec<u8>), &'static str> {
        let now_command = self.read_8bit(bus, self.program_counter);
        let now_inst = self.instruction_set[now_command as usize];
        let mut fetched_bytes: Vec<u8> = Vec::new();
        fetched_bytes.push(now_command);
        trace!("CPU got command: {}, instruction: {now_inst}", common::number_to_hex(now_command, true));
        trace!(
            "Working with {} bytes of data from {}",
            now_inst.op_name().as_digit(),
            common::number_to_hex(self.program_counter, true)
        );

        match now_inst.op_name() {
            CPUInstByte::One(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_1_byte(inst_entry, bus);

                if matches!(self.state, CpuState::Stopped) {
                    return Err("CPU was stopped by STP instruction")
                }
            },
            CPUInstByte::Two(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let next_data_byte = self.read_8bit(bus, self.program_counter);
                fetched_bytes.push(next_data_byte);
                let target_byte = self.conv_1byte_address(now_inst.memory_type(), next_data_byte, bus);
                trace!("Current data value: {}", common::number_to_hex(target_byte, true));
                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_2_byte(bus, inst_entry, target_byte);
            },
            CPUInstByte::Three(inst_entry) => {
                fetched_bytes.push(self.read_8bit(bus, self.program_counter.wrapping_add(1)));
                fetched_bytes.push(self.read_8bit(bus, self.program_counter.wrapping_add(2)));
                self.program_counter = self.program_counter.wrapping_add(1);
                let next_value = self.read_16bit(bus, self.program_counter);
                let target_address = self.conv_2byte_address(now_inst.memory_type(), next_value, bus);
                trace!("Address:{} -> {}",common::number_to_hex(next_value, true), common::number_to_hex(target_address, true));
                self.program_counter = self.program_counter.wrapping_add(2);
                self.execute_inst_3_byte(bus, inst_entry, target_address);
            },
            CPUInstByte::NoOp => {
                error!(
                    "Trying to parse NoOp instruction at {} with hex {}",
                    common::number_to_hex(self.program_counter, true),
                    common::number_to_hex(now_command, true)
                );
                return Err("NoOp parsed")
            }
        }

        Ok((now_inst, fetched_bytes))
    }
}
