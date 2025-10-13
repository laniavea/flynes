use std::fs;
use std::ffi::OsString;

use better_assertions::{inst_assert, inst_assert_eq};
use log::{error, info, debug};

use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::mappers;
use crate::common::DataSizes;

const PRGROM_BYTES_IN_UNITS: usize = DataSizes::Size16K.to_bytes();
const CHRROM_BYTES_IN_UNITS: usize = DataSizes::Size8K.to_bytes();

const CB1_VRAM_LAYOUT: usize = 3;
const CB1_TRAINER: usize = 2;
const CB1_BATTERY_RAM: usize = 1;
const CB1_MIRRORING_TYPE: usize = 0;

#[derive(Debug, Clone, Copy)]
enum NesCartridgeError {
    NoNESHeader,
    NESHeaderMustBeZero,
    PRGROMIncorrectNumber,
}

impl std::fmt::Display for NesCartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoNESHeader => write!(f, "File doesn't contains NES header"),
            Self::NESHeaderMustBeZero => write!(f, "Some of bytes that must be zero are not, check your nes file"),
            Self::PRGROMIncorrectNumber => write!(f, "NES file should contains at least 1 PRGROM, check your file"),
        }
    }
}

impl std::error::Error for NesCartridgeError {}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum MirroringType {
    Horizontal,
    Vertical,
}

impl MirroringType {
    fn from_bool(input_bool_status: bool) -> MirroringType {
        if input_bool_status {
            MirroringType::Vertical
        } else {
            MirroringType::Horizontal
        }
    }
}

impl std::fmt::Display for MirroringType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "Horizontal"),
            Self::Vertical => write!(f, "Vertical"),
        }
    }

}

#[derive(Debug, Clone, Copy)]
/// Contains information about ROM from NES header file (NES 1.0)
struct NESHeaderInfo {
    /// Number of 16kB ROB banks (PRG ROM)
    number_prgrom_banks: u8,
    /// Number of 8kB ROB banks (CHR ROM)
    _number_chrrom_banks: u8,
    /// Rom mapper type
    _mapper_type: u8,
    /// Four-screen VRAM layout
    _four_screen_vram: bool,
    /// Trainer include status
    trainer_include: bool,
    /// Battery packed RAM to store saves
    _battery_packed_ram: bool,
    /// Mirroring status
    _mirroring_type: MirroringType,
    /// Size of PRG RAM in 8kB units
    _prgram_size: u8,
}

impl NESHeaderInfo {
    fn from_bytes(input_header_data: &[u8]) -> Result<NESHeaderInfo, NesCartridgeError> {
        inst_assert_eq!(input_header_data.len(), 16);
        info!("Starting parsing NES header from the file");

        if input_header_data[..4] != *"NES\x1A".as_bytes() {
            error!("The NES header doesn't start with 'NES^Z' part");
            return Err(NesCartridgeError::NoNESHeader);
        }

        let number_prgrom_banks = input_header_data[4];
        let number_chrrom_banks = input_header_data[5];
        debug!("Number of PRG ROM banks (16kB): {number_prgrom_banks}");
        debug!("Number of CHR ROM banks (8kB): {number_chrrom_banks}");

        let first_control_byte = input_header_data[6];
        let four_screen_vram = is_bit_set(first_control_byte, CB1_VRAM_LAYOUT);
        let trainer_include = is_bit_set(first_control_byte, CB1_TRAINER);
        let battery_packed_ram = is_bit_set(first_control_byte, CB1_BATTERY_RAM);
        let mirroring_type = MirroringType::from_bool(
            is_bit_set(first_control_byte, CB1_MIRRORING_TYPE)
        );
        debug!("Four-screen VRAM layout: {four_screen_vram}");
        debug!("Trainer available: {trainer_include}");
        debug!("Battery packed RAM available: {battery_packed_ram}");
        debug!("Mirroring type: {mirroring_type}");

        let second_control_byte = input_header_data[7];

        let mapper_type_low_part = first_control_byte >> 4;
        inst_assert!(mapper_type_low_part < 0b0001_0000);
        let mapper_type: u8 = (second_control_byte & 0b1111_0000) + mapper_type_low_part;
        debug!("Mapper type: {mapper_type}");

        let prgram_size = input_header_data[8];
        debug!("PRG RAM size (in 8kB units): {prgram_size}");

        if input_header_data[9..].iter().any(|el| *el != 0) {
            error!("At least one byte at the end(10-16 bytes) of NES header are not zeros");
            return Err(NesCartridgeError::NESHeaderMustBeZero);
        }

        let header_info = NESHeaderInfo {
            number_prgrom_banks,
            _number_chrrom_banks: number_chrrom_banks,
            _mapper_type: mapper_type,
            _four_screen_vram: four_screen_vram,
            trainer_include,
            _battery_packed_ram: battery_packed_ram,
            _mirroring_type: mirroring_type,
            _prgram_size: prgram_size,
        };

        info!("NES header parsed successfully");

        header_info.validate()?;

        Ok(header_info)
    }

    pub fn validate(&self) -> Result<(), NesCartridgeError> {
        info!("Starting NES header validation");

        if self.number_prgrom_banks == 0 || self.number_prgrom_banks > 2 {
            error!("Incorrect number of PRG ROM banks (16kb): {}", self.number_prgrom_banks);
            return Err(NesCartridgeError::PRGROMIncorrectNumber);
        }
        debug!("Number of PRGROM: validated successfully");

        info!("NES header was validated successfully");

        Ok(())
    }

    pub fn create_modules(&self, file_data: &[u8]) -> (Cpu, Bus) {
        info!("Start of creating NES modules");
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        debug!("CPU and bus initialization complete");

        if self.trainer_include {
            panic!("No trainer support");
        }

        debug!("Number of banks: {}", self.number_prgrom_banks);

        let prg_rom_size = PRGROM_BYTES_IN_UNITS * self.number_prgrom_banks as usize;
        let chr_rom_size = CHRROM_BYTES_IN_UNITS * self._number_chrrom_banks as usize;

        let prg_data = &file_data[0..prg_rom_size];
        let chr_data = &file_data[prg_rom_size..prg_rom_size+chr_rom_size];
        
        let mapper = mappers::create_mapper(
            self._mapper_type,
            bus.memory_mut(),
            prg_data,
            chr_data
        ).unwrap(); // TODO: Remove unwrap

        bus.set_mapper(mapper);

        debug!("PRG-ROM successfully wrote");
        cpu.init_pc(&mut bus);
        info!("NES modules were created successfully");

        (cpu, bus)
    }
}

fn is_bit_set(input_byte: u8, target_bit: usize) -> bool {
    inst_assert!(target_bit < 8);
    input_byte & (0b0000_0001 << target_bit) == (0b0000_0001 << target_bit)
}

pub fn read_nes_file(path_to_nes_file: OsString) -> Result<(Cpu, Bus), Box<dyn std::error::Error>> {
    info!("Start of processing NES file");
    let nes_file_data = fs::read(path_to_nes_file)?;

    let nes_header = if nes_file_data.len() < 17 {
        return Err(Box::new(NesCartridgeError::NoNESHeader));
    } else {
        &nes_file_data[0..16]
    };
    debug!("NES file read successfully");

    let header_info = NESHeaderInfo::from_bytes(nes_header)?;
    let (cpu, bus) = header_info.create_modules(&nes_file_data[16..]);

    Ok((cpu, bus))
}

pub fn load_raw_commands(bus: &mut Bus, commands: Vec<u8>) {
    debug!("Loading raw commands");

    let prg_data = append_missed_data(commands, DataSizes::Size16K.to_bytes());

    let mapper = mappers::create_mapper(
        0,
        bus.memory_mut(),
        &prg_data,
        &vec![0u8; DataSizes::Size8K.to_bytes()],
    ).unwrap(); //TODO: Remove this unwrap

    bus.set_mapper(mapper);
}

fn append_missed_data(mut data_to_append: Vec<u8>, target_size: usize) -> Vec<u8> {
    match target_size.cmp(&data_to_append.len()) {
        std::cmp::Ordering::Less => {
            error!("Provided data is bigger than target");
            data_to_append
        },
        std::cmp::Ordering::Equal => data_to_append,
        std::cmp::Ordering::Greater => {
            let number_of_bytes_to_fill = target_size - data_to_append.len();
            data_to_append.extend(vec![0u8; number_of_bytes_to_fill].iter());
            data_to_append
        }
    }
}
