#[inline(always)]
pub fn update_zero_and_neg_flags(cpu_status: u8, op_result: u8) -> u8 {
    if op_result & 0b1000_0000 == 0b1000_0000 {
        // set neg -> set non_zero
        (cpu_status | 0b0000_0010) & 0b1011_1111
    } else if op_result == 0 {
        // set zero -> set non_neg
        (cpu_status | 0b0100_0000) & 0b1111_1101
    } else {
        // set non_zero -> set non_neg
        (cpu_status & 0b1011_1111) & 0b1111_1101
    }
}
