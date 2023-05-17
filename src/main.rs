#![allow(unused)]
mod medium;
mod easy;

use easy::nthprime::*;
use exercism_rust_track::*;
use medium::{clock::*, anagram::anagrams_for, space_age::*};

use crate::easy::beer_song::{verse, sing};

fn main() {
    let output = nth(5);
    println!("{}", output);
}
