pub fn reply(message: &str) -> &str {
    let mut message_str = message.trim();
    let mut message_filtered: String = message_str.to_string().chars().filter(|c| c.is_alphabetic()).collect();
    println!("{}", message_filtered);
    if (message_str.len() > 0) {
        if (message_filtered.len()> 0 && message_filtered.to_ascii_uppercase() == message_filtered) {
            let last_char = message_str.chars().last().unwrap();
            if (last_char == '?') {
                return "Calm down, I know what I'm doing!";
            } else {
                return "Whoa, chill out!";
            }
        }

        let last_char = message_str.chars().last().unwrap();
        if (last_char == '?') {
            return "Sure.";
        }
        return "Whatever.";
    } else {
        return "Fine. Be that way!";
    }
}
