#![allow(unused)]
mod easy;
mod hard;
mod medium;

use std::{time::Instant, collections::HashMap};

use easy::matching_brackets::*;
use exercism_rust_track::*;
use medium::sublist::*;


fn main() {
    let now = Instant::now();
    
    let input = "{}[";
    println!("{:?}", brackets_are_balanced(input));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
