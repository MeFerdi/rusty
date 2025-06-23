pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let n = num_str.len() as u32;
    let sum = num_str.chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(n))
        .sum::<u32>();
    sum == num
}