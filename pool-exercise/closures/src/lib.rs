pub fn first_fifty_even_square() -> Vec<i32>{
    (1..)
    .map(|x| x * 2)
    .map(|x| x * x)
    .take(50)
    .collect::<Vec<i32>>()
}
