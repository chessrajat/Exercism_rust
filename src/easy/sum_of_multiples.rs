use std::collections::HashSet;



pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples:HashSet<u32> = HashSet::new();
    for val in factors {
        if *val == 0 {
            break;
        }
        let mut i = 1;
        loop {
            let multiple = val * i;
            if multiple >= limit {
                break;
            }
            unique_multiples.insert(multiple);
            i += 1;
        }
        
    }

    return unique_multiples.iter().sum::<u32>();
}

pub fn sum_of_multiples2(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|i| factors.iter().any(|&f| f != 0 && i % f == 0))
        .sum()
}
