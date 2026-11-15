use std::fs;

use flynes_core::bus::Bus;
use flynes_core::cartridges;
use flynes_core::common::number_to_hex;
use flynes_core::cpu::Cpu;

const LOG_VERSION: u8 = 2;

fn main() {
    pretty_env_logger::init();

    let (mut cpu_unit, mut bus_unit) =
        match cartridges::read_nes_file("./../../roms/nestest.nes".into()) {
            Ok(modules) => modules,
            Err(err) => {
                println!("Error occured, see log");
                println!("Error: {err}");
                return;
            }
        };

    cpu_unit.set_pc(0xC000);

    let mut all_file_data = String::new();

    let file_name = match LOG_VERSION {
        1 => {
            for _not_iter in 0..8991
            /*8991*/
            {
                let cpu_before_exec_state = cpu_unit;
                if let Err(e) = cpu_unit.execute_operation(&mut bus_unit, true) {
                    eprintln!("CPU Execution failed: {e}");
                    break;
                }

                let st = generate_log_string_v1(&cpu_before_exec_state, &cpu_unit);
                all_file_data += &format!("{st}\n");
            }

            String::from("nestest_v1.log")
        }
        2 => {
            for _not_iter in 0..8991
            /*8991*/
            {
                let cpu_before_exec_state = cpu_unit;
                if let Err(e) = cpu_unit.execute_operation(&mut bus_unit, true) {
                    eprintln!("CPU Execution failed: {e}");
                    break;
                }
                let st = generate_log_string_v2(&cpu_before_exec_state, &cpu_unit, &bus_unit);
                all_file_data += &format!("{st}\n");
            }

            String::from("nestest_v2.log")
        }
        _ => unreachable!("Unknown log version"),
    };

    let log_result = all_file_data.trim_end();
    fs::write(file_name, log_result).unwrap();
    println!("LOG READY!");
}

fn generate_log_string_v1(cpu: &Cpu, updated_cpu: &Cpu) -> String {
    let pc = cpu.program_counter();
    let pc_str = number_to_hex(pc, false);

    let fetched_bytes: Vec<String> = updated_cpu
        .debug_info()
        .expect("CPU was executed without debug info collection")
        .fetched_bytes()
        .iter()
        .map(|el| number_to_hex(*el, false))
        .collect();

    let bytes_str: String = fetched_bytes.join(" ");

    let all_regs = cpu.get_registers_state();
    let (reg_a, reg_x, reg_y) = (all_regs[0], all_regs[1], all_regs[2]);
    let reg_a_str = format!("A:{}", number_to_hex(reg_a, false));
    let reg_x_str = format!("X:{}", number_to_hex(reg_x, false));
    let reg_y_str = format!("Y:{}", number_to_hex(reg_y, false));

    let cpu_status = cpu.cpu_status();
    let cpu_status_str = format!("P:{}", number_to_hex(cpu_status, false));

    let sp = cpu.stack_pointer();
    let sp_str = format!("SP:{}", number_to_hex(sp, false));

    format!(
        "{pc_str}\t{bytes_str}\t{reg_a_str}\t{reg_x_str}\t{reg_y_str}\t{cpu_status_str}\t{sp_str}"
    )
}

fn generate_log_string_v2(cpu: &Cpu, updated_cpu: &Cpu, bus: &Bus) -> String {
    let cpu_debug_info = updated_cpu
        .debug_info()
        .as_ref()
        .expect("CPU was executed without debug info collection");

    let bus_debug_info = bus
        .debug_info()
        .as_ref()
        .expect("CPU was executed without debug info collection");

    let pc = cpu.program_counter();
    let pc_str = number_to_hex(pc, false);

    let fetched_bytes: Vec<String> = cpu_debug_info
        .fetched_bytes()
        .iter()
        .map(|el| number_to_hex(*el, false))
        .collect();
    let bytes_str: String = fetched_bytes.join(" ");

    let op_disassembly = flynes_core::debug_info::cpu_debug_str(cpu_debug_info, bus_debug_info);

    let all_regs = cpu.get_registers_state();
    let (reg_a, reg_x, reg_y) = (all_regs[0], all_regs[1], all_regs[2]);
    let reg_a_str = format!("A:{}", number_to_hex(reg_a, false));
    let reg_x_str = format!("X:{}", number_to_hex(reg_x, false));
    let reg_y_str = format!("Y:{}", number_to_hex(reg_y, false));

    let cpu_status = cpu.cpu_status();
    let cpu_status_str = format!("P:{}", number_to_hex(cpu_status, false));

    let sp = cpu.stack_pointer();
    let sp_str = format!("SP:{}", number_to_hex(sp, false));

    format!(
        "{pc_str}\t{bytes_str}\t{op_disassembly}\t{reg_a_str}\t{reg_x_str}\t{reg_y_str}\t{cpu_status_str}\t{sp_str}"
    )
}
