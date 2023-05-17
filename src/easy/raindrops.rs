
pub fn raindrops(n: u32) -> String {
    let mut output = String::new();
    let num = Some(n);
    match (n % 3, n % 5, n % 7 ) {
        (0, 0, 0) => output.push_str("PlingPlangPlong"),
        (0, _, 0) => output.push_str("PlingPlong"),
        (0, 0, _) => output.push_str("PlingPlang"),
        (_, 0, 0) => output.push_str("PlangPlong"),
        (0, _, _) => output.push_str("Pling"),
        (_, 0, _) => output.push_str("Plang"),
        (_, _, 0) => output.push_str("Plong"),
        _ => output.push_str(format!("{}", n).as_str()),
    }
    return output;
}

pub fn raindrops2(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;
    let mut rez = String::new();
    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }
    if rez.is_empty() { rez = x.to_string(); }
    rez
}