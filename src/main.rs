#![allow(unused)]
mod medium;
mod easy;

use easy::proverb::*;
use exercism_rust_track::*;

use crate::easy::beer_song::{verse, sing};

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = vec!["nail", "shoe", "horse"];
    let output = build_proverb2(&input);
    println!("{:?}", output);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
