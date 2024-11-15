mod memory;
mod instructions;

// -- CPU STATUS --
// ----- Flags -----
// 7 6 5 4 3 2 1 0
// N V _ B D I Z C
//
// N - Negative
// V - Overflow 
// _ - Should always be 1
// B - Break
// D - Decimal
// I - Interrupt
// Z - Zero
// C - Carry
//
// reg a - accumulator
// reg x - x register
// reg y - y register
//
// stack_pointer - refers to EMPTY cell inside memory's stack part
// program_counter - refers to place where next instructions be executed
//
// operations - all posible op codes for 6502, more in instructions.rs
// memory - CPU memory, more in memory.rs

pub const NEGATIVE_FLAG: u8 = 7;
pub const OVERFLOW_FLAG: u8 = 6;
pub const BREAK_FLAG: u8 = 4;
pub const DECIMAL_FLAG: u8 = 3;
pub const INTERRUPT_FLAG: u8 = 2;
pub const ZERO_FLAG: u8 = 1;
pub const CARRY_FLAG: u8 = 0;

#[derive(Debug, Clone)]
pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    cpu_status: u8,
    stack_pointer: u8,
    program_counter: u16,
    operations: [Option<instructions::Operation>; 256],
    memory: [u8; 0x10000],
}

impl Default for Cpu {
    //TODO: change default to values that should be (mb 0) and memory
    fn default() -> Cpu {
        Self {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            cpu_status: 0b00100100,
            stack_pointer: u8::default(),
            program_counter: u16::default(),
            operations: instructions::init_all_operations(),
            memory: [0u8; 0x10000],
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
            memory: [0u8; 0x10000],
        }
    }

    pub fn run_cpu(&mut self, commands: Vec<u8>) {
        let mut now_command_id = 0;
        while now_command_id < commands.len() {
            let now_operations = self.operations[commands[now_command_id] as usize]
                .unwrap_or_else(|| panic!("Unknown operation - {}", commands[now_command_id]));

            let data_ref: u16 = match now_operations.bytes() {
                1 => 0,
                2 => {
                    now_command_id += 1;
                    self.ref_to_memory_by_address(commands[now_command_id] as u16, now_operations.memory_type())
                },
                3 => {
                    let now_data = (commands[now_command_id + 1] as u16 + (commands[now_command_id + 2] as u16)) << 8;
                    now_command_id += 2;
                    self.ref_to_memory_by_address(now_data, now_operations.memory_type())
                },
                _ => { unreachable!() }
            };

            self.do_insturction(data_ref, now_operations.op_type());

            self.print_regs();
            self.print_zero_page();
            self.print_flags();

            now_command_id += 1;
        }
    }
}

impl Cpu {
    pub fn get_flag(&self, flag_to_find: u8) -> bool {
        // 7 6 5 4 3 2 1 0
        // N V _ B D I Z C
        if flag_to_find > 7 { panic!("Unreachable flag tried to be setted for cpu status") }
        (self.cpu_status >> flag_to_find) % 2 == 1
    }

    pub fn set_flag(&mut self, flag_to_set: u8, value_to_set: bool) {
        // 7 6 5 4 3 2 1 0
        // N V _ B D I Z C

        if flag_to_set > 7 { panic!("Unreachable flag tried to be setted for cpu status") }
        if value_to_set {
            self.cpu_status |= 0b0000_0001 << flag_to_set;
        } else {
            self.cpu_status &= !(0b0000_0001 << flag_to_set);
        }
    }

    pub fn get_cpu_status(&self) -> u8 {
        self.cpu_status
    }
}

impl Cpu {
    pub fn print_regs(&self) {
        println!("-------------Registers-------------");
        println!("REG A: {}\nREG X: {}\nREG Y: {}", self.reg_a, self.reg_x, self.reg_y);
        println!("-----------------------------------");
    }

    pub fn print_zero_page(&self) {
        println!("-------------Zero Page-------------");
        for i in 0..16 {
            for j in 0..16 {
                print!("{:?}, ", self.read_mem(i * 16 + j))
            }
            println!();
        }
        println!("-----------------------------------");
    }

    pub fn print_stack(&self) {
        println!("---------------Stack---------------");
        for i in 0..16 {
            for j in 0..16 {
                print!("{:?}, ", self.read_mem(0x1FF - (i * 16 + j)))
            }
            println!();
        }
        println!("-----------------------------------");
    }

    pub fn print_flags(&self) {
        println!("-------------Status-------------");

        let flags: [String; 8] = [
            String::from("Carry\t"),
            String::from("Zero\t"),
            String::from("Interrupt"),
            String::from("Decimal"),
            String::from("Break\t"),
            String::from("_\t"),
            String::from("Overflow"),
            String::from("Negative")
        ];

        let st_val = 1;

        for (now_id, now_flag) in flags.iter().enumerate() {
            if self.cpu_status & (st_val << now_id) == st_val << now_id {
                println!("{} \t-> {}", now_flag, 1)
            } else {
                println!("{} \t-> {}", now_flag, 0)
            }
        }

        println!("-----------------------------------");
    }
}

#[test]
fn test_get_flag() {
    let mut cpu = Cpu {
        cpu_status: 0b0000_0000,
        ..Default::default()
    };

    for now_i in 0..=7 {
        cpu.set_flag(now_i, true);
        assert!(cpu.get_flag(now_i));

        cpu.set_flag(now_i, false);
        assert!(!cpu.get_flag(now_i));

        cpu.set_flag(now_i, true);
        assert!(cpu.get_flag(now_i));
    }

    assert_eq!(cpu.cpu_status, 0b1111_1111);
}
