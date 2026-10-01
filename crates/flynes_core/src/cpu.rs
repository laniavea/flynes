use better_assertions::{fast_assert, inst_assert_eq};
use log::{debug, error, info, warn};

use crate::bus::{Bus, InterruptStatus};
use crate::common;
use crate::debug_info::CpuDebugInfo;
use crate::memory::MemoryType;
use instructions::{CPUInstByte, Operation};

pub mod instructions;

const RESET_ON_CPU_EXEC_ERR: bool = true;

const CARRY_FLAG: usize = 0;
const ZERO_FLAG: usize = 1;
const INTERRUPT_FLAG: usize = 2;
const DECIMAL_FLAG: usize = 3;
const BREAK_FLAG: usize = 4;
const UNUSED_FLAG: usize = 5;
const OVERFLOW_FLAG: usize = 6;
const NEGATIVE_FLAG: usize = 7;

static INSTRUCTION_SET: [Operation; 256] = instructions::init_cpu_operations().0;
static INSTRUCTION_COUNT: usize = instructions::init_cpu_operations().1;

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
    exec_instructions: usize,
    debug_info: Option<CpuDebugInfo>,
}

impl Default for Cpu {
    fn default() -> Cpu {
        inst_assert_eq!(
            INSTRUCTION_COUNT,
            INSTRUCTION_SET
                .iter()
                .filter(|i| !((i.cycles() == 0) && matches!(i.op_name(), CPUInstByte::NoOp)))
                .count()
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
            exec_instructions: 0,
            debug_info: None,
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

    pub fn set_regs(&mut self, reg_a: Option<u8>, reg_x: Option<u8>, reg_y: Option<u8>) {
        if let Some(reg_a) = reg_a {
            self.reg_a = reg_a
        }
        if let Some(reg_x) = reg_x {
            self.reg_x = reg_x
        }
        if let Some(reg_y) = reg_y {
            self.reg_y = reg_y
        }
    }

    pub fn set_cpu_status(&mut self, cpu_status: u8) {
        self.cpu_status = cpu_status
    }

    pub fn set_stack_pointer(&mut self, stack_pointer: u8) {
        self.stack_pointer = stack_pointer
    }

    pub fn set_exec_cycles(&mut self, exec_cycles: usize) {
        self.exec_cycles = exec_cycles
    }

    pub fn set_exec_instructions(&mut self, exec_instructions: usize) {
        self.exec_instructions = exec_instructions
    }

    pub fn init_sp(&mut self, new_stack_pointer: u8) {
        self.stack_pointer = new_stack_pointer
    }

    pub fn stack_pointer_mut(&mut self) -> &mut u8 {
        &mut self.stack_pointer
    }

    pub fn get_registers_state(&self) -> [u8; 3] {
        [self.reg_a, self.reg_x, self.reg_y]
    }

    pub fn cpu_status(&self) -> u8 {
        self.cpu_status
    }

    pub fn stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn debug_info(&self) -> &Option<CpuDebugInfo> {
        &self.debug_info
    }

    pub fn exec_cycles(&self) -> usize {
        self.exec_cycles
    }

    pub fn exec_instructions(&self) -> usize {
        self.exec_instructions
    }
}

impl Cpu {
    pub fn conv_1byte_address(&mut self, mt: MemoryType, value: u8, bus: &mut Bus) -> u16 {
        fast_assert!(
            [
                MemoryType::Immediate,
                MemoryType::ZeroPage,
                MemoryType::ZeroPageX,
                MemoryType::ZeroPageY,
                MemoryType::Relative,
                MemoryType::IndirectX,
                MemoryType::IndirectY,
            ]
            .contains(&mt)
        );

        match mt {
            MemoryType::Immediate => self.program_counter,
            MemoryType::ZeroPage => value as u16,
            MemoryType::ZeroPageX => {
                let value = value.wrapping_add(self.reg_x);
                value as u16
            }
            MemoryType::ZeroPageY => {
                let value = value.wrapping_add(self.reg_y);
                value as u16
            }
            MemoryType::Relative => self.program_counter,
            MemoryType::IndirectX => {
                let value = value.wrapping_add(self.reg_x);
                self.read_16bit_zp_wrap(bus, value as u16)
            }
            MemoryType::IndirectY => {
                let value_data = self.read_16bit_zp_wrap(bus, value as u16);
                value_data.wrapping_add(self.reg_y as u16)
            }
            _ => unreachable!(),
        }
    }

    pub fn conv_2byte_address(&mut self, mt: MemoryType, value: u16, bus: &mut Bus) -> u16 {
        fast_assert!(
            [
                MemoryType::Absolute,
                MemoryType::AbsoluteX,
                MemoryType::AbsoluteY,
                MemoryType::Indirect,
            ]
            .contains(&mt)
        );

        match mt {
            MemoryType::Absolute => value,
            MemoryType::AbsoluteX => value.wrapping_add(self.reg_x as u16),
            MemoryType::AbsoluteY => value.wrapping_add(self.reg_y as u16),
            MemoryType::Indirect => self.read_16bit_jmp_bug(bus, value),
            _ => unreachable!(),
        }
    }
}

impl Cpu {
    #[inline(always)]
    pub fn bytes_to_16bit_le_order(first_byte: u8, second_byte: u8) -> u16 {
        ((second_byte as u16) << 8) + (first_byte as u16)
    }

    #[inline(always)]
    pub fn read_8bit<T>(&mut self, bus: &mut Bus, data_ref: T) -> u8
    where
        T: Into<usize> + Copy,
    {
        bus.read_8bit_cpu(data_ref, &self.exec_cycles)
    }

    #[inline(always)]
    pub fn write_8bit<T>(&mut self, bus: &mut Bus, data_ref: T, data_value: u8)
    where
        T: Into<usize> + Copy,
    {
        bus.write_8bit_cpu(data_ref, data_value, &self.exec_cycles);
    }

    #[inline(always)]
    pub fn peek_8bit<T>(&mut self, bus: &mut Bus, data_ref: T) -> u8
    where
        T: Into<usize> + Copy,
    {
        bus.peek_8bit_cpu(data_ref)
    }

    pub fn read_16bit(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        let requested_byte = self.read_8bit(bus, requested_address);
        let next_byte = self.read_8bit(bus, requested_address.wrapping_add(1));

        Self::bytes_to_16bit_le_order(requested_byte, next_byte)
    }

    pub fn read_16bit_zp_wrap(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        if requested_address == 0x00FF {
            let first_byte = self.read_8bit(bus, 0x00FFusize);
            let second_byte = self.read_8bit(bus, 0x0000usize);
            return Self::bytes_to_16bit_le_order(first_byte, second_byte);
        }

        self.read_16bit(bus, requested_address)
    }

    pub fn read_16bit_jmp_bug(&mut self, bus: &mut Bus, requested_address: u16) -> u16 {
        if requested_address & 0x00FF == 0x00FF {
            let first_byte = self.read_8bit(bus, requested_address);
            let second_byte = self.read_8bit(bus, requested_address & 0xFF00);
            return Self::bytes_to_16bit_le_order(first_byte, second_byte);
        }

        self.read_16bit(bus, requested_address)
    }
}

impl Cpu {
    fn trigger_nmi(&mut self, bus: &mut Bus) {
        bus.memory_mut()
            .stack_push_16bit(self.program_counter, &mut self.stack_pointer);
        let _new_cpu_status = self.cpu_status;
        //TODO: Implement NMI trigger
    }
}

impl Cpu {
    pub fn run_cpu(&mut self, bus: &mut Bus) {
        debug!(
            "Running CPU with next PC: {}",
            common::number_to_hex(self.program_counter, true)
        );

        let max_number_of_operations = 100_000_000;
        let mut now_oper: usize = 0;

        while now_oper < max_number_of_operations {
            match self.execute_operation(bus, false) {
                Ok(_) => now_oper += 1,
                Err(err_msg) => {
                    if RESET_ON_CPU_EXEC_ERR {
                        warn!("RESET ON ERROR: {err_msg}");
                        self.set_pc(0xC000);
                    } else {
                        error!("Error while CPU execution: {err_msg}");
                        break;
                    }
                }
            }
        }

        info!("Leaving RUN CPU on {now_oper}");
    }

    pub fn execute_operation(
        &mut self,
        bus: &mut Bus,
        need_debug: bool,
    ) -> Result<(), &'static str> {
        bus.set_debug(need_debug);

        let op_byte = self.read_8bit(bus, self.program_counter);
        let op_inst = self.instruction_set[op_byte as usize];

        match op_inst.op_name() {
            CPUInstByte::One(inst_entry) => {
                if need_debug {
                    self.debug_info =
                        Some(CpuDebugInfo::new(self.program_counter, op_inst, None, None));
                }

                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_1_byte(inst_entry, bus);

                if matches!(self.state, CpuState::Stopped) {
                    error!("CPU was stopped by STP instruction");
                    return Err("STP instruction was called");
                }
            }
            CPUInstByte::Two(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let data_byte = self.read_8bit(bus, self.program_counter);
                let target_byte = self.conv_1byte_address(op_inst.memory_type(), data_byte, bus);

                if need_debug {
                    self.debug_info = Some(CpuDebugInfo::new(
                        self.program_counter,
                        op_inst,
                        Some(data_byte),
                        None,
                    ));
                }

                self.program_counter = self.program_counter.wrapping_add(1);
                self.execute_inst_2_byte(bus, inst_entry, target_byte);
            }
            CPUInstByte::Three(inst_entry) => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let data_bytes = self.read_16bit(bus, self.program_counter);

                if need_debug {
                    self.debug_info = Some(CpuDebugInfo::new(
                        self.program_counter,
                        op_inst,
                        Some((data_bytes & 0b0000_0000_1111_1111) as u8),
                        Some((data_bytes >> 8) as u8),
                    ));
                }
                let target_address =
                    self.conv_2byte_address(op_inst.memory_type(), data_bytes, bus);

                self.program_counter = self.program_counter.wrapping_add(2);
                self.execute_inst_3_byte(bus, inst_entry, target_address);
            }
            CPUInstByte::NoOp => {
                error!(
                    "Trying to parse NoOp instruction at {} with hex {}",
                    common::number_to_hex(self.program_counter, true),
                    common::number_to_hex(op_byte, true)
                );
                return Err("NoOp parsed");
            }
        }

        if matches!(bus.interrupt_status(), InterruptStatus::NMI) {
            self.trigger_nmi(bus);
            bus.set_interrupt_status(InterruptStatus::None);
        }

        self.exec_cycles += op_inst.cycles() as usize;
        self.exec_instructions += 1;

        Ok(())
    }

    pub fn decode_next_instructions(&mut self, bus: &mut Bus, pc: u16, instr_num: usize) -> Vec<String> {
        let mut target_pc = pc;
        let mut res_decode: Vec<String> = vec![];

        for _ in 0..instr_num {
            let op_byte = self.peek_8bit(bus, target_pc);
            let op_inst = self.instruction_set[op_byte as usize];

            let bytes: Vec<u8> = match op_inst.op_name() {
                CPUInstByte::One(_) => vec![],
                CPUInstByte::Two(_) => {
                    target_pc = target_pc.wrapping_add(1);
                    vec![(self.peek_8bit(bus, target_pc))]
                }
                CPUInstByte::Three(_) => {
                    target_pc = target_pc.wrapping_add(1);
                    let mut bytes = vec![self.peek_8bit(bus, target_pc)];
                    target_pc = target_pc.wrapping_add(1);
                    bytes.push(self.peek_8bit(bus, target_pc));
                    bytes
                }
                CPUInstByte::NoOp => {
                    error!(
                        "Trying to parse NoOp instruction at {} with hex {}",
                        common::number_to_hex(pc, true),
                        common::number_to_hex(op_byte, true)
                    );
                    vec![]
                }
            };

            res_decode.push(op_inst.raw_disassembly(pc, &bytes));
        }

        res_decode
    }
}
