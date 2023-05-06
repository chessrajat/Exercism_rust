use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_string(input: &str) -> String {
    let mut output = String::new();
    for val in input.graphemes(true).rev(){
        output.push_str(&val.to_string());
    }
    return output
}