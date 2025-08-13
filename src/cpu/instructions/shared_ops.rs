use better_assertions::inst_assert;

use crate::cpu::{NEGATIVE_FLAG, ZERO_FLAG};

const NEGATIVE_FLAG_BIT: u8 = 0b0000_0001 << NEGATIVE_FLAG;
const ZERO_FLAG_BIT: u8 = 0b0000_0001 << ZERO_FLAG;

#[inline(always)]
pub fn update_zero_and_neg_flags(cpu_status: &mut u8, op_result: u8) {
    if op_result == 0 {
        *cpu_status |= ZERO_FLAG_BIT;
        *cpu_status &= !NEGATIVE_FLAG_BIT; 
    } else {
        *cpu_status = (*cpu_status & !NEGATIVE_FLAG_BIT) | (op_result & NEGATIVE_FLAG_BIT);
        *cpu_status &= !ZERO_FLAG_BIT;
    }
}

#[inline(always)]
pub fn is_flag_set(cpu_status: &u8, flag_to_check: usize) -> bool {
    inst_assert!((0..=7).contains(&flag_to_check));
    cpu_status & (0b0000_0001 << flag_to_check) == (0b0000_0001 << flag_to_check)
}

#[inline(always)]
pub fn set_flag(cpu_status: &mut u8, flag_to_check: usize, is_one: bool) {
    inst_assert!((0..=7).contains(&flag_to_check));
    if is_one {
        *cpu_status |= 0b0000_0001 << flag_to_check
    } else {
        *cpu_status &= !(0b0000_0001 << flag_to_check)
    }
}


#[inline(always)]
pub fn transfer_bit(transfer_to: &mut u8, transfer_from: &u8, bit_num: usize) {
    inst_assert!((0..=7).contains(&bit_num));
    let tr_bit = 0b0000_0001 << bit_num;
    *transfer_to = (*transfer_to & !tr_bit) | (transfer_from & tr_bit);
}

#[test]
fn test_update_zero_and_neg_flags() {
    for i in 0..=255 {
        for result_value in [0b1111_1111, 0b1000_0000, 0b0111_1111, 0b0000_0001, 0b0000_0000] {
            cmp_res(i, result_value);
        }
    }

    fn cmp_res(old_status: u8, result_value: u8) {
        let mut new_status = old_status;
        update_zero_and_neg_flags(&mut new_status, result_value);

        let preserved_mask = !(ZERO_FLAG_BIT | NEGATIVE_FLAG_BIT);
        assert_eq!(new_status & preserved_mask, old_status & preserved_mask, "Unrelated flags modified");
        assert_eq!(new_status & NEGATIVE_FLAG_BIT, result_value & NEGATIVE_FLAG_BIT, "NEGATIVE flag incorrect");
        assert_eq!(new_status & ZERO_FLAG_BIT == ZERO_FLAG_BIT, result_value == 0, "ZERO flag incorrect");
    }
}

#[test]
fn test_is_flag_set() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    for _ in 0..1000 {
        let bool_arr: [bool; 8] = rng.random();

        let mut random_value = 0b0000_0000;
        for (now_shift, now_bool) in bool_arr.iter().enumerate() {
            if *now_bool {
                random_value |= 0b0000_0001 << now_shift
            }
        }

        for (now_id, now_flag) in bool_arr.iter().enumerate() {
            assert_eq!(is_flag_set(&random_value, now_id), *now_flag);
        }
    }
}

#[test]
// Depends on is_flag_set
fn test_set_flag() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    for _ in 0..1000 {
        let bool_arr: [bool; 8] = rng.random();

        let mut random_value = 0b0000_0000;
        for (now_shift, now_bool) in bool_arr.iter().enumerate() {
            if *now_bool {
                random_value |= 0b0000_0001 << now_shift
            }
        }

        for (now_id, now_flag) in bool_arr.iter().enumerate() {
            let mut new_value = random_value;

            set_flag(&mut new_value, now_id, !now_flag);
            for (now_id_ck, now_flag_ck) in bool_arr.iter().enumerate() {
                if now_id_ck != now_id {
                    assert_eq!(is_flag_set(&new_value, now_id_ck), *now_flag_ck);
                } else {
                    assert_eq!(is_flag_set(&new_value, now_id_ck), !*now_flag_ck);
                }
            }

            set_flag(&mut new_value, now_id, *now_flag);
            for (now_id_ck, now_flag_ck) in bool_arr.iter().enumerate() {
                assert_eq!(is_flag_set(&new_value, now_id_ck), *now_flag_ck);
            }
        }
    }
}

#[test]
// Depends on is_flag_set
fn test_transfer_bit() {
    use rand::{SeedableRng, Rng};
    use rand::rngs::StdRng;

    let mut rng: StdRng = StdRng::seed_from_u64(42);

    for _ in 0..1000 {
        let bool_arr: [bool; 8] = rng.random();

        let mut random_value = 0b0000_0000;
        for (now_shift, now_bool) in bool_arr.iter().enumerate() {
            if *now_bool {
                random_value |= 0b0000_0001 << now_shift
            }
        }

        let mut random_target: u8 = rng.random::<u8>();

        for (now_id, now_flag) in bool_arr.iter().enumerate() {
            transfer_bit(&mut random_target, &random_value, now_id);
            assert_eq!(is_flag_set(&random_target, now_id), *now_flag)
        }

        assert_eq!(random_target, random_value);
    }
}
