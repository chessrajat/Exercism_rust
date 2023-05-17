pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }

    return u64::pow(2, s-1);
}

pub fn total() -> u64 {
    return (1..=64).map(|x| u64::pow(2, x-1)).sum();
}

// pub fn total() -> u64 {
//     (1..65)
//         .map(square)
//         .fold(0, Add::add)
// }

pub fn is_leap_year(year: u64) -> bool {
    if year % 100 == 0 && year % 400  != 0 {
        return false;
    }
    if year%4 == 0 {
        return true;
    }
    return false;
}

// pub fn is_leap_year(year: u64) -> bool {
//     match (year % 400, year % 100, year % 4) {
//         (0, _, _) => true,
//         (_, 0, _) => false,
//         (_, _, 0) => true,
//         _ => false
//     }
// }
