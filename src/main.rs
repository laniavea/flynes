use log::info;

pub mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::default();

    let test_commands: Vec<u8> = vec![0xA9, 0xFF, 0x85, 0x00, 0xA9, 0x00, 0xA9, 0x01, 0x4A];

    cpu.run_cpu(test_commands);
    info!("CPU module created with next size: {:?} bytes", size_of_val(&cpu));
}
