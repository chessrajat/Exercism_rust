pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1..=n).fold(0, |a, b| a + b);
    return sum * sum;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let sum_square = (1..=n).map(|a| a * a).sum();
    return sum_square;
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}

// pub fn sum_of_squares(n: u64) -> u64 {
//     n * (n + 1) * (2 * n + 1) / 6
// }
// pub fn square_of_sum(n: u64) -> u64 {
//     (n * (n + 1) / 2).pow(2)
// }
// pub fn difference(number: u64) -> u64 {
//     square_of_sum(number) - sum_of_squares(number)
// }