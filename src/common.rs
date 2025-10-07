pub enum DataSizes {
    Size2K,
    Size4K,
    Size8K,
    Size16K,
    Size32K,
}

impl DataSizes {
    pub const fn to_bytes(&self) -> usize {
        match self {
            DataSizes::Size2K => 1024 * 2,
            DataSizes::Size4K => 1024 * 4,
            DataSizes::Size8K => 1024 * 8,
            DataSizes::Size16K => 1024 * 16,
            DataSizes::Size32K => 1024 * 32,
        }
    }
}

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
