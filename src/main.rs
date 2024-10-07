use log::info;

pub mod cpu;

fn main() {
    let cpu = cpu::Cpu::default();
    info!("CPU module created with next size: {:?} bytes", size_of_val(&cpu));
}
