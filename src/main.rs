extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod cpu;

fn main() {
    pretty_env_logger::init();

    info!("Starting main function");

    test_smth_fn();

    let mut cpu = cpu::Cpu::default();
    info!("CPU module created with next size: {:?} bytes", size_of_val(&cpu));

    let test_commands: Vec<u8> = vec![0xA9, 0xFF, 0x85, 0x00, 0xA9, 0x00, 0xA9, 0x01, 0x4A];

    cpu.run_cpu(test_commands);
}

fn test_smth_fn() {
    // let mut cpu = cpu::Cpu::default();
    //
    // use std::time::Instant;
    //
    // let now_t = Instant::now();
    //
    // let test_commands: Vec<u8> = vec![0xC0, 0xC0];
    // cpu.run_cpu(test_commands);
    //
    // let mut seed: u32 = 52;
    // for i in 0..100_000_000 {
    //     let test_commands: Vec<u8> = vec![0xC0, 0xC0];
    //     cpu.run_cpu(test_commands);
    //     let mut num = seed;
    //     num ^= num << 13;
    //     num ^= num >> 17;
    //     num ^= num << 5;
    //     seed = num;
    // }
    //
    // let mark = now_t.elapsed();
    // println!("TIME: {:?}, end_status: {}", mark, cpu.get_cpu_status());

}
