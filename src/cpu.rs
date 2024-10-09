mod memory;
mod instructions;

// ----- Flags -----
// 7 6 5 4 3 2 1 0
// N V _ B D I Z C
//
// N - Negative
// V - Overflow 
// B - Break
// D - Decimal
// I - Interrupt
// Z - Zero
// C - Carry

#[derive(Debug, Clone)]
pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    cpu_status: u8,
    _stack_pointer: u8,
    _program_counter: u16,
    operations: [Option<instructions::Operation>; 256],
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
            _stack_pointer: u8::default(),
            _program_counter: u16::default(),
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
            _stack_pointer: u8::default(),
            _program_counter: u16::default(),
            operations: instructions::init_all_operations(),
            memory: [0u8; 0xFFFF],
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
    pub fn print_regs(&self) {
        println!("-------------Registers-------------");
        println!("REG A: {}\nREG X: {}\nREG Y: {}", self.reg_a, self.reg_x, self.reg_y);
        println!("-----------------------------------");
    }

    pub fn print_zero_page(&self) {
        println!("-------------Zero Page-------------");
        for i in 0..16 {
            for j in 0..16 {
                print!("{:?}, ", self.memory[i * 16 + j])
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
