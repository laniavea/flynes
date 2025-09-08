pub fn number_to_hex(value: u8, with_hex_iden: bool) -> String {
    let mut res = format!("{value:#x}").to_uppercase()[2..].to_string();

    if res.len() == 1 {
        res = format!("0{res}").to_string()
    }

    if with_hex_iden {
        res = format!("0x{res}");
    }

    res
}
