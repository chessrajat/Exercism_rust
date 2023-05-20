#![allow(unused)]
mod medium;
mod easy;

use easy::sum_of_multiples::*;
use exercism_rust_track::*;

use crate::easy::beer_song::{verse, sing};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let output = sum_of_multiples(10_000, &[2, 3, 5, 7, 11]);
    println!("{:?}", output);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
