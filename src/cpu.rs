use std::collections::HashMap;

mod memory;
mod instructions;

#[derive(Debug, Clone)]
pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    cpu_status: u8,
    stack_pointer: u8,
    program_counter: u16,
    operations: HashMap<u8, instructions::Operation>,
    memory: [u8; 0xFFFF],
}

impl Default for Cpu {
    //TODO: change default to values that should be (mb 0) and memory
    fn default() -> Cpu {
        Self {
            reg_a: u8::default(),
            reg_x: u8::default(),
            reg_y: u8::default(),
            cpu_status: u8::default(),
            stack_pointer: u8::default(),
            program_counter: u16::default(),
            operations: instructions::init_all_operations(),
            memory: [0u8; 0xFFFF],
        }
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        //TODO: change default to values that should be (mb 0) and memory
        Self {
            reg_a: u8::default(),
            reg_x: u8::default(),
            reg_y: u8::default(),
            cpu_status: u8::default(),
            stack_pointer: u8::default(),
            program_counter: u16::default(),
            operations: instructions::init_all_operations(),
            memory: [0u8; 0xFFFF],
        }
    }

    pub fn main_loop() {

    }
}
