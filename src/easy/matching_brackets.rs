pub fn brackets_are_balanced(string: &str) -> bool {
    let mut characters: Vec<char> = Vec::new();
    for c in string.chars() {
        if is_bracket(c) == true {
            if c == ')' {
                let curr_char = characters.pop().unwrap_or_else(|| return '0');
                if curr_char != '(' || curr_char == '0' {
                    return false;
                }
            } else if c == '}' {
                let curr_char = characters.pop().unwrap_or_else(|| return '0');
                if curr_char != '{' || curr_char == '0' {
                    return false;
                }
            } else if c == ']' {
                let curr_char = characters.pop().unwrap_or_else(|| return '0');
                if curr_char != '[' || curr_char == '0' {
                    return false;
                }
            } else {
                characters.push(c);
            }
        }
    }
    if characters.len() != 0 {
        return false;
    }
    return true;
}

fn is_bracket(bracket: char) -> bool {
    let brackets = String::from("()[]{}");
    match brackets.find(bracket) {
        Some(index) => return true,
        None => return false,
    }
}
