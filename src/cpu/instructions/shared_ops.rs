pub fn update_zero_and_neg_flags(cpu_status: u8, op_result: u8) -> u8 {
    let temp_cpu_status = if op_result == 0 {
        cpu_status | 0b0100_0000
    } else {
        cpu_status & 0b1011_1111
    };

    if op_result & 0b1000_0000 == 0b1000_0000 {
        temp_cpu_status | 0b0000_0010
    } else {
        temp_cpu_status & 0b1111_1101
    }

}
