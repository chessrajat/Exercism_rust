pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code == "0" {
        return false;
    }
    let code: Result<Vec<u32>, bool> = code
        .chars()
        .map(|c| match (c.to_digit(10)) {
            Some(digit) => Ok(digit),
            None => return Err(false),
        })
        .collect::<Result<Vec<u32>, bool>>();
    let code = match code {
        Ok(code) => code,
        Err(_) => return false,
    };

    let mut temp_code: Vec<u32> = Vec::new();
    let mut gapper = false;
    for val in code.iter().rev() {
        if gapper {
            // perform operations
            let double_val = val * 2;
            if (double_val > 9) {
                temp_code.push(double_val - 9)
            } else {
                temp_code.push(double_val);
            }
            gapper = false;
        } else {
            temp_code.push(*val);
            gapper = true;
        }
    }
    let total_sum: u32 = temp_code.iter().sum();
    if total_sum % 10 == 0 {
        return true;
    } else {
        return false;
    }
}

fn is_valid2(s: &str) -> bool {
    let mut sum = 0;
    let mut len = 0;
    for (i, c) in s.chars().rev().filter(|&c| c != ' ').enumerate() {
        len += 1;
        match (i % 2, c.to_digit(10)) {
            (1, Some(x)) if x > 4 => sum += x * 2 - 9,
            (1, Some(x)) => sum += x * 2,
            (0, Some(x)) => sum += x,
            (_, _) => return false,
        }
    }
    return false;
}
