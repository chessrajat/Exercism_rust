

pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut output: Vec<u64> = Vec::new();
    let mut i = 2;
    loop {
        if n % i == 0 {
            output.push(i);
            n /= i;
        }else {
            i += 1;
        }
        if n == 1{
            break;
        }
    }
    return output;
}

pub fn factors2(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    match (2..n+1).find(|x| n%x == 0) {
        Some(x) => {
            result.push(x);
            result.append(&mut factors(n/x));
        },
        None => {}
    }
    return result;
}


