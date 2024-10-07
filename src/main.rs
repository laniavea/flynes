use log::info;

pub mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::default();

    let test_commands: Vec<u8> = vec![0xA9, 0xFF];

    cpu.run_cpu(test_commands);
    info!("CPU module created with next size: {:?} bytes", size_of_val(&cpu));
}
