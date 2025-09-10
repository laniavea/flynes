pub fn number_to_hex<T: std::fmt::UpperHex + Sized> (value: T, with_hex_iden: bool) -> String {
    let mut res: String = format!("{value:X}");

    if res.len() != size_of_val(&value) * 2 {
        let null_prefix: String = "0".repeat((size_of_val(&value) * 2) - res.len());
        res = format!("{null_prefix}{res}")
    }

    if with_hex_iden {
        res = format!("0x{res}");
    }

    res
}
