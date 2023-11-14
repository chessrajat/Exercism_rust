#![allow(unused)]
mod easy;
mod hard;
mod medium;

use std::{time::Instant, collections::HashMap};

use easy::bob::*;
use exercism_rust_track::*;
use medium::sublist::*;


fn main() {
    let now = Instant::now();
    
    let input: &str = "1 ,2 ,3";
    let result = reply(input);
    println!("{}", result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
