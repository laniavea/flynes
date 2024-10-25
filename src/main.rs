use log::info;

pub mod cpu;

fn main() {
    test_smth_fn();

    let mut cpu = cpu::Cpu::default();

    let test_commands: Vec<u8> = vec![0xA9, 0xFF, 0x85, 0x00, 0xA9, 0x00, 0xA9, 0x01, 0x4A];

    cpu.run_cpu(test_commands);
    info!("CPU module created with next size: {:?} bytes", size_of_val(&cpu));
}

fn test_smth_fn() {
    // let mut cpu = cpu::Cpu::default();
    //
    // use std::time::Instant;
    //
    // let now_t = Instant::now();
    //
    // let mut seed: u32 = 52;
    // for i in 0..100_000_000 {
    //     let mut num = seed;
    //     num ^= num << 13;
    //     num ^= num >> 17;
    //     num ^= num << 5;
    //     seed = num;
    //
    //     if i % 5 == 0 {
    //         cpu.set_flag((num % 8) as u8, num % 2 == 1 )
    //     } else {
    //         let _ = cpu.get_flag((num % 8) as u8);
    //     }
    // }
    //
    // let mark = now_t.elapsed();
    // println!("TIME: {:?}, end_status: {}", mark, cpu.get_cpu_status());

}
