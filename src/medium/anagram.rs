use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut output = HashSet::new();
    let word_hash = generate_anagram_hash(word.to_lowercase().as_str());
    for possible in possible_anagrams {
        let possible_word = generate_anagram_hash(possible.to_lowercase().as_str());
        if possible_word == word_hash && word.to_lowercase() != possible.to_lowercase() {
            output.insert(possible.to_owned());
        }
    }
    return output;
}

fn generate_anagram_hash(word: &str) -> HashMap<char, i32> {
    let mut h: HashMap<char, i32> = HashMap::new();
    for c in word.chars() {
        *h.entry(c).or_insert(0) += 1;
    }
    // for c in word.chars() {
    //     if h.contains_key(&c) {
    //         h.insert(c, h[&c]+1);
    //     }else{
    //         h.insert(c, 1);
    //     }
    // }

    return h;
}
