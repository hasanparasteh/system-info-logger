pub static BYTE_TO_MB_DIVISON_AMOUNT: u64 = 1_000_000;

pub fn bytes_to_mb(size: u64) -> u64 {
    size / BYTE_TO_MB_DIVISON_AMOUNT
}