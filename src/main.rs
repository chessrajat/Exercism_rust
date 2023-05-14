#![allow(unused)]
mod medium;
mod easy;

use easy::armstrong_numbers::is_armstrong_number;
use exercism_rust_track::*;
use medium::{clock::*, anagram::anagrams_for, space_age::*};

fn main() {
    let result = is_armstrong_number(3_999_999_999);
    println!("{}", result);
}
