#[cfg(feature = "add")]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "sub")]
pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}


