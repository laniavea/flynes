use std::fs;

use flynes_core::cartridges;
use flynes_core::common::number_to_hex;
use flynes_core::cpu::Cpu;

const LOG_VERSION: u8 = 1;

fn main() {
    pretty_env_logger::init();

    let (mut cpu_unit, mut memory_unit) =
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
    let mut file_name: Option<String> = None;

    assert!(file_name.is_none());
    match LOG_VERSION {
        1 => {
            for _not_iter in 0..8991
            /*8991*/
            {
                let old_cpu = cpu_unit;
                let fected_bytes = match cpu_unit.execute_operation(&mut memory_unit, true) {
                    Ok(_) => {
                        if let Some(cpu_debug_info) = cpu_unit.debug_info() {
                            cpu_debug_info.as_bytes()
                        } else {
                            eprintln!("Debug turned off");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("CPU Execution failed: {e}");
                        break;
                    }
                };
                let st = generate_log_string_v1(&old_cpu, fected_bytes);
                all_file_data += &format!("{st}\n");
            }

            file_name = Some(String::from("nestest_v1.log"))
        }
        _ => unreachable!("Unknown log version"),
    }

    let log_result = all_file_data.trim_end();
    fs::write(file_name.unwrap(), log_result).unwrap();
    println!("LOG READY!");
}

fn generate_log_string_v1(cpu: &Cpu, fetched_bytes: Vec<u8>) -> String {
    let pc = cpu.program_counter();
    let pc_str = number_to_hex(pc, false);

    let mut bytes_str: String = String::new();
    for i in 0..3 {
        if let Some(fb) = fetched_bytes.get(i) {
            bytes_str.push_str(number_to_hex(*fb, false).as_str());
            if i != 2 {
                bytes_str.push(' ');
            }
        }
    }

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
