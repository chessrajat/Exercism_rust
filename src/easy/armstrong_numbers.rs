pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }
    let num_str = format!("{}", num);
    let num_length = String::from(&num_str).len() as u32;
    let mut output: u32 = 0;
    for val in num_str.chars() {
        let int_val = char::to_digit(val, 10).unwrap();
        output = output
            .checked_add(u32::pow(int_val, num_length))
            .unwrap_or_else(|| {
                return 0;
            });
    }
    if output == num {
        return true;
    } else {
        return false;
    }
}

// pub fn is_armstrong_number(num: u32) -> bool {
//     if num == 0 {
//         return true;
//     }
//     let num = u64::from(num);
//     let digit_count = (num as f64).log10() as u32 + 1;
//     let mut candidate = num as u64;
//     let mut total = 0;
//     while candidate > 0 {
//         total += (candidate % 10).pow(digit_count);
//         candidate /= 10;
//     }
//     num == total
// }
