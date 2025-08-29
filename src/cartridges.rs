use std::fs;
use std::ffi::OsString;

use crate::cpu::Cpu;
use crate::memory::Memory;

#[derive(Debug)]
enum NesCartridgeError {
    NoNESHeader,
}

impl std::fmt::Display for NesCartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoNESHeader => write!(f, "File doesn't contains NES header")
        }
    }
}

impl std::error::Error for NesCartridgeError {}

pub fn read_nes_file(path_to_nes_file: OsString) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let nes_file_data = fs::read(path_to_nes_file)?;
    Ok(nes_file_data)
}

pub fn init_from_nes_file(cpu: &mut Cpu, mem: &mut Memory) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
