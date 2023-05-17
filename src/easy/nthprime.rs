
pub fn nth(n: u32) -> u32 {
    let mut primes:Vec<u32> = Vec::new();
    let mut i = 2;
    while primes.len() as u32 != n + 1 {
        if is_prime2(i) {
            primes.push(i);
        }
        i += 1;
    }
    println!("{:?}", primes);
    return primes[n as usize];
}

fn is_prime(num: u32) -> bool {
    for i in 2..num {
        if num%i == 0 {
            return false;
        }
    }
    return true;
}

fn is_prime2(num: u32) -> bool {
    !(2..num).any(|x| num%x == 0)
}
