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

    pub fn print_regs(&self) {
        println!("--------------");
        println!("REG A: {}\nREG X: {}\nREG Y: {}", self.reg_a, self.reg_x, self.reg_y);
        println!("--------------");
    }

    pub fn run_cpu(&mut self, commands: Vec<u8>) {
        let mut now_command_id = 0;
        let mut counter = 0;
        while now_command_id < commands.len() && counter < 1_000_000_000 {
            let now_operations = self.operations[commands[now_command_id] as usize].expect("Unknown operation");

            let readed_data = match now_operations.bytes() {
                1 => self.read_memory(0, now_operations.memory_type()),
                2 => {
                    now_command_id += 1;
                    self.read_memory(commands[now_command_id] as u16, now_operations.memory_type())
                },
                3 => {
                    let now_data = (commands[now_command_id + 1] as u16 + (commands[now_command_id + 2] as u16)) << 8;
                    self.read_memory(now_data, now_operations.memory_type())
                },
                _ => { unreachable!() }
            };

            self.do_insturction(readed_data, now_operations.op_type());

            // self.print_regs();

            now_command_id += 1;
            counter += 1;
            now_command_id = 0;
        }
    }
}
