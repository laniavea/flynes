#[inline(always)]
pub fn update_zero_and_neg_flags(cpu_status: u8, op_result: u8) -> u8 {
    // If first bit is one -> true value is negative
    if op_result >= 128 {
        // set neg -> set non_zero
        (cpu_status | 0b1000_0000) & 0b1111_1101
    } else if op_result == 0 {
        // set not neg -> set zero
        (cpu_status & 0b0111_1111) | 0b0000_0010
    } else {
        // set non_zero -> set non_neg
        cpu_status & 0b0111_1101
    }
}

#[inline(always)]
pub fn update_carry_flag(cpu_status: u8, value: u8) -> u8 {
    if value % 2 == 0 {
        cpu_status & 0b1111_1110
    } else {
        cpu_status | 0b0000_0001
    }
}

#[test]
fn test_update_zero_and_neg_flags() {
    assert_eq!(update_zero_and_neg_flags(0b0000_0000, 255), 0b1000_0000);
    assert_eq!(update_zero_and_neg_flags(0b0000_0000, 128), 0b1000_0000);
    assert_eq!(update_zero_and_neg_flags(0b0000_0000, 127), 0b0000_0000);
    assert_eq!(update_zero_and_neg_flags(0b0000_0000, 1), 0b0000_0000);
    assert_eq!(update_zero_and_neg_flags(0b0000_0000, 0), 0b0000_0010);
}

#[test]
fn test_update_carry_flag() {
    assert_eq!(update_carry_flag(0b0000_0000, 1), 0b0000_0001);
    assert_eq!(update_carry_flag(0b0000_0000, 127), 0b0000_0001);
    assert_eq!(update_carry_flag(0b0000_0000, 128), 0b0000_0000);
    assert_eq!(update_carry_flag(0b0000_0000, 0), 0b0000_0000);
}
