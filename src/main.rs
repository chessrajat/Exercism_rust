#![allow(unused)]
mod easy;
mod hard;
mod medium;

use std::{time::Instant, collections::HashMap};

use easy::sum_of_multiples::*;
use exercism_rust_track::*;
use medium::sublist::*;

use crate::{
    easy::beer_song::{sing, verse},
    hard::parallel_letter_frequency::frequency,
    medium::{luhn::is_valid, minesweeper::annotate},
};

fn main() {
    let now = Instant::now();

    // let input = [
    //     "Freude schöner Götterfunken",
    //     "Tochter aus Elysium,",
    //     "Wir betreten feuertrunken,",
    //     "Himmlische, dein Heiligtum!",
    //     "Deine Zauber binden wieder",
    //     "Was die Mode streng geteilt;",
    //     "Alle Menschen werden Brüder,",
    //     "Wo dein sanfter Flügel weilt.",
    // ];
    // let result = frequency(&input, 3);
    // println!("Result: {:?}", result);

    let result:HashMap<u32,u32> = hashmap!(1 => 2,);
    println!("Result: {:?}", result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
