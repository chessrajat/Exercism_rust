#![allow(unused)]
mod medium;

use exercism_rust_track::*;
use medium::{clock::*, anagram::anagrams_for};

fn main() {
    let a = "bAnana";
    let b = ["enlists", "google", "inlets", "banana", "abnaan"];
    let out = anagrams_for(a, &b);
    println!("{:?}", out);
}
