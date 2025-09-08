use log::info;

pub mod cpu;
pub mod memory;
pub mod cartridges;
pub mod common;

const WORKFLOW_MODE: u8 = 2;

fn main() {
    pretty_env_logger::init();

    match WORKFLOW_MODE {
        1 => temp_unit(),
        2 => test_rom(),
        _ => unreachable!(),
    }
}

fn temp_unit() {
    let mut cpu_unit = cpu::Cpu::default();
    info!("CPU unit initializated");
    let mut memory_unit = memory::Memory::default();
    info!("Memory unit initializated");

    use rand;
    let mut commands: Vec<u8> = vec![0xCAu8; 0x8000];
    commands.iter_mut().for_each(|i| *i = rand::random::<u8>());
    commands[0x7FFE] = 0x00;
    commands[0x7FFF] = 0x80;

    commands[0x0000] = 0xA9;
    commands[0x0001] = 0xA9;
    memory_unit.load_rom(&commands);

    cpu_unit.init_pc(&memory_unit);

    use std::time::Instant;
    let t = Instant::now();

    cpu_unit.run_cpu(&mut memory_unit);

    let tt = t.elapsed();
    println!("Elapsed {tt:?}");
}

fn test_rom() {
    let (
        mut cpu_unit,
        mut memory_unit
    ) = match cartridges::read_nes_file("./roms/nestest.nes".into()) {
        Ok(modules) => modules,
        Err(err) => {
            println!("Error occured, see log");
            println!("Error: {err}");
            return
        }
    };

    cpu_unit.set_pc(0xC000);

    cpu_unit.run_cpu(&mut memory_unit);
}
