use std::{
    collections::HashMap,
    result,
    sync::{Arc, Barrier},
    thread::{self, JoinHandle},
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::new();

    let mut thread_pool = Vec::new();

    if input.is_empty() {
        return result;
    }
    let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    for line in input {
        let answer = thread::spawn(move || calculate_frequency(&line));
        thread_pool.push(answer);
    }

    for thread in thread_pool {
        let frequencies = thread.join().unwrap();
        for (char, count) in frequencies {
            *result.entry(char).or_insert(0) += count;
        }
    }

    return result;
}

fn calculate_frequency(line: &str) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for ch in line.to_lowercase().chars() {
        if (ch.is_alphabetic()) {
            result
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    return result;
}

// try rayon
