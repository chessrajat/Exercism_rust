use unicode_segmentation::UnicodeSegmentation;

pub fn reverse_string(input: &str) -> String {
    let mut output = String::new();
    for val in input.graphemes(true).rev() {
        output.push_str(&val.to_string());
    }
    return output;
}

use time::Duration;
use time::PrimitiveDateTime as DateTime;

// ----------create date time
// use time::macros::datetime;
// let start_date = datetime!(2015-01-24 22:00:00);
// ----------cargo.toml file
// time = { version = "0.3.21", features = ["macros"] }

pub fn after(start: DateTime) -> DateTime {
    return start + Duration::seconds(1000000000);
}


