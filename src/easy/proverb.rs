
pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb_list: Vec<String> = Vec::new();
    if list.len() == 0 {
        return String::new();
    }
    for i in 0..list.len() - 1 {
        let word = list[i];
        let next_word = list[i + 1];
        let proverb = format!("For want of a {word} the {next_word} was lost.", word=word, next_word=next_word);
        proverb_list.push(proverb);
    }

    proverb_list.push(format!("And all for the want of a {}.", list[0]));

    return proverb_list.join("\n")
}

use std::iter::once;
pub fn build_proverb2(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(word) => list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", word)))
            .collect(),
    }
}