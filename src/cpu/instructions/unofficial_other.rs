use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::cpu::CpuState;

impl Cpu {
    /// SHX (SXA, XAS); And reg_x with high byte of the address + 1 then store
    pub fn op_shx(&mut self, bus: &mut Bus, data_ref: u16) {
        let data_to_write = self.reg_x & ((data_ref >> 8) as u8).wrapping_add(1);
        bus.write_8bit_cpu(data_ref, data_to_write)
    }

    /// SHY (SYA, SAY); And reg_y with high byte of the address + 1 then store
    pub fn op_shy(&mut self, bus: &mut Bus, data_ref: u16) {
        let data_to_write = self.reg_y & ((data_ref >> 8) as u8).wrapping_add(1);
        bus.write_8bit_cpu(data_ref, data_to_write)
    }

    /// STP (KIL, JAM, HLT) operations; Processor lock up
    pub fn op_stp(&mut self) {
        self.state = CpuState::Stopped;
    }

    /// XAA (ANE) operations; Unstable instruction, see nesdev
    pub fn op_xaa(&mut self, bus: &mut Bus, data_ref: u16) {
        let read_data = bus.read_8bit_cpu(data_ref);
        self.reg_a = (self.reg_a & self.reg_x) & read_data;
    }

    /// AHX (AXA, SHA) operation; and reg_x with reg_a then and with 7
    pub fn op_ahx(&mut self, bus: &mut Bus, data_ref: u16) {
        bus.write_8bit_cpu(data_ref, (self.reg_a & self.reg_x) & 7)
    }

    /// TAS (XAS, SHS) operation; and between reg_x and accumulator. Store result in SP. Then and
    /// SP with HIGH(memory) and story it in memory
    pub fn op_tas(&mut self, bus: &mut Bus, data_ref: u16) {
        self.stack_pointer = self.reg_x & self.reg_a;
        let data_to_write = self.stack_pointer & (((data_ref >> 8) as u8).wrapping_add(1));
        bus.write_8bit_cpu(data_ref, data_to_write);
    }

    /// LAX immediate (ATX, LXA, OAL) operations; and memory with accumulator. Then transfer reg_a
    /// to reg_x
    pub fn op_lax_other_ver(&mut self, bus: &mut Bus, data_ref: u16) {
        self.reg_a &= bus.read_8bit_cpu(data_ref);
        self.reg_x = self.reg_a;
    }

    /// LAS (LAR, LAE) operations; and memory with SP. transfer result to reg_a, reg_x and SP
    pub fn op_las(&mut self, bus: &mut Bus, data_ref: u16) {
        self.stack_pointer &= bus.read_8bit_cpu(data_ref);
        self.reg_x = self.stack_pointer;
        self.reg_a = self.stack_pointer;
    }
}
