pub fn stars(n: u32) -> String {
    let count = 2_u32.pow(n);
    "*".repeat(count as usize)
}
