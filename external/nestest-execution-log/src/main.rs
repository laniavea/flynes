use flynes::cpu::Cpu;
use flynes::cartridges;

fn main() {
    pretty_env_logger::init();

    let (
        mut cpu_unit,
        mut memory_unit
    ) = match cartridges::read_nes_file("./../../roms/nestest.nes".into()) {
        Ok(modules) => modules,
        Err(err) => {
            println!("Error occured, see log");
            println!("Error: {err}");
            return
        }
    };

    cpu_unit.set_pc(0xC000);

    for _not_iter in 0..10000 {
        let (op, fected_bytes) = cpu_unit.execute_cpu_iteration_info(&mut memory_unit).unwrap();
        generate_log_string_v1(&cpu_unit, fected_bytes);
    }

}

fn generate_log_string_v1(cpu: &Cpu, fetched_bytes: Vec<u8>) {
    let pc = cpu.get_program_counter();
    let pc_str = &format!("{pc:#x}").to_uppercase()[2..];

    let mut bytes_str: String = String::new();
    for i in 0..3 {
        if let Some(fb) = fetched_bytes.get(i) {
            bytes_str.push_str(number_to_hex(*fb).as_str());
            if i == 2 {
                bytes_str.push('\t');
            } else {
                bytes_str.push(' ');
            }
        } else {
            bytes_str.push('\t')
        }
    }

    let (reg_a, reg_x, reg_y) = cpu.get_registers_state();
    let reg_a_str = format!("A:{}", number_to_hex(reg_a));
    let reg_x_str = format!("X:{}", number_to_hex(reg_x));
    let reg_y_str = format!("Y:{}", number_to_hex(reg_y));

    let sp = cpu.get_stack_pointer();
    let sp_str = format!("SP:{}", number_to_hex(sp));

    println!("{pc_str}\t{bytes_str}\t{reg_a_str}\t{reg_x_str}\t{reg_y_str}\t{sp_str}");
}

fn number_to_hex(value: u8) -> String {
    let mut res = format!("{value:#x}").to_uppercase()[2..].to_string();
    // if res.len() == 1 {
    //     res = format!("0{res}").to_string()
    // }

    res
}
