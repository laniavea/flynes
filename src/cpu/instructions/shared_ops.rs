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
pub fn update_carry_flag_by_7_bit(cpu_status: u8, value: u8) -> u8 {
    if value & 0b1000_0000 != 0b1000_0000 {
        cpu_status & 0b1111_1110
    } else {
        cpu_status | 0b0000_0001
    }
}

#[inline(always)]
pub fn update_carry_flag(cpu_status: u8, value: u8) -> u8 {
    if value & 0b0000_0001 != 0b0000_0001 {
        cpu_status & 0b1111_1110
    } else {
        cpu_status | 0b0000_0001
    }
}

#[inline(always)]
pub fn set_overflow_flag(cpu_status: u8, is_one: bool) -> u8 {
    if is_one {
        cpu_status | 0b0100_0000
    } else {
        cpu_status & 0b1011_1111
    }
}

#[inline(always)]
pub fn set_interrupt_flag(cpu_status: u8, is_one: bool) -> u8 {
    if is_one {
        cpu_status | 0b0000_0100
    } else {
        cpu_status & 0b1111_1011
    }
}

#[inline(always)]
pub fn set_decimal_flag(cpu_status: u8, is_one: bool) -> u8 {
    if is_one {
        cpu_status | 0b0000_1000
    } else {
        cpu_status & 0b1111_0111
    }
}

#[inline(always)]
pub fn set_carry_flag(cpu_status: u8, is_one: bool) -> u8 {
    if is_one {
        cpu_status | 0b0000_0001
    } else {
        cpu_status & 0b1111_1110
    }
}

// #[inline(always)]
// pub fn get_flag_inl(cpu_status: u8, flag_to_find: u8) -> bool {
//     // 7 6 5 4 3 2 1 0
//     // N V _ B D I Z C
//     // Read more in cpu.rs
//     match flag_to_find {
//         0 => cpu_status & 0b0000_0001 == 0b0000_0001,
//         1 => cpu_status & 0b0000_0010 == 0b0000_0010,
//         2 => cpu_status & 0b0000_0100 == 0b0000_0100,
//         3 => cpu_status & 0b0000_1000 == 0b0000_1000,
//         4 => cpu_status & 0b0001_0000 == 0b0001_0000,
//         5 => unreachable!(),
//         6 => cpu_status & 0b0100_0000 == 0b0100_0000,
//         7 => cpu_status & 0b1000_0000 == 0b1000_0000,
//         _ => unreachable!(),
//     }
// }

#[inline(always)]
pub fn get_flag_inl(cpu_status: u8, flag_to_find: u8) -> bool {
    // 7 6 5 4 3 2 1 0
    // N V _ B D I Z C
    // Read more in cpu.rs
    if flag_to_find > 7 { panic!("Unreachable flag tried to be setted for cpu status") }
    (cpu_status >> flag_to_find) % 2 == 1
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

#[test]
fn test_update_carry_flag_by_7_bit() {
    assert_eq!(update_carry_flag_by_7_bit(0b0000_0000, 1), 0b0000_0000);
    assert_eq!(update_carry_flag_by_7_bit(0b0000_0000, 127), 0b0000_0000);
    assert_eq!(update_carry_flag_by_7_bit(0b0000_0000, 128), 0b0000_0001);
    assert_eq!(update_carry_flag_by_7_bit(0b0000_0000, 255), 0b0000_0001);
    assert_eq!(update_carry_flag_by_7_bit(0b0000_0000, 0), 0b0000_0000);
}

#[test]
fn test_set_interupt_flag() {
    assert_eq!(set_interrupt_flag(0b0000_0000, true), 0b0000_0100);
    assert_eq!(set_interrupt_flag(0b0000_0000, false), 0b0000_0000);
    assert_eq!(set_interrupt_flag(0b1111_1111, true), 0b1111_1111);
    assert_eq!(set_interrupt_flag(0b1111_1111, false), 0b1111_1011);
}

#[test]
fn test_set_decimal_flag() {
    assert_eq!(set_decimal_flag(0b0000_0000, true), 0b0000_1000);
    assert_eq!(set_decimal_flag(0b0000_0000, false), 0b0000_0000);
    assert_eq!(set_decimal_flag(0b1111_1111, true), 0b1111_1111);
    assert_eq!(set_decimal_flag(0b1111_1111, false), 0b1111_0111);
}

#[test]
fn test_set_carry_flag() {
    assert_eq!(set_carry_flag(0b0000_0000, true), 0b0000_0001);
    assert_eq!(set_carry_flag(0b0000_0000, false), 0b0000_0000);
    assert_eq!(set_carry_flag(0b1111_1111, true), 0b1111_1111);
    assert_eq!(set_carry_flag(0b1111_1111, false), 0b1111_1110);
}

#[test]
fn test_set_overflow_flag() {
    assert_eq!(set_overflow_flag(0b0000_0000, true), 0b0100_0000);
    assert_eq!(set_overflow_flag(0b0000_0000, false), 0b0000_0000);
    assert_eq!(set_overflow_flag(0b1111_1111, true), 0b1111_1111);
    assert_eq!(set_overflow_flag(0b1111_1111, false), 0b1011_1111);
}

#[test]
fn test_get_flag_inl() {
    let first_status = 0b1010_1010;
    let second_status = 0b0101_0101;

    for now_i in 0..=7 {
        if now_i == 5 { continue }
        if now_i % 2 == 0 {
            assert!(!get_flag_inl(first_status, now_i));
        } else {
            assert!(get_flag_inl(first_status, now_i));
        }

        if now_i % 2 == 0 {
            assert!(get_flag_inl(second_status, now_i));
        } else {
            assert!(!get_flag_inl(second_status, now_i));
        }
    }

    assert_eq!(!first_status, second_status);
}
