pub fn verse(n: u32) -> String {
    if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    } else if n == 1 {
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    let bottle_string:String;
    if n-1 == 1 {
        bottle_string = String::from("bottle")
    }else{
        bottle_string = String::from("bottles")
    }
    return format!("{bottle_value} bottles of beer on the wall, {bottle_value} bottles of beer.\nTake one down and pass it around, {one_less_bottle} {bottle_string} of beer on the wall.\n",bottle_value=n, one_less_bottle=n - 1, bottle_string=bottle_string);
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for val in (end..=start).rev() {
        let out = verse(val);
        song.push_str(&out);
        song.push_str("\n")
    }
    return song.strip_suffix("\n").unwrap().to_string();
}

#[test]
#[ignore]
fn test_song_8_6() {
    assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}


// pub fn verse(n: i32) -> String {
//     match n {
//         0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
//         1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
//         _ => format!(
//             "{0} {1} of beer on the wall, {0} {1} of beer.\nTake one down and pass it around, {2} {3} of beer on the wall.\n",
//             n, bottles(n), n-1, bottles(n-1)
//         )
//     }
// }
// pub fn sing(start: i32, end: i32) -> String {
//     assert!(start > end);
//     let verses: Vec<String> = (end..start+1).map(verse).rev().collect();
//     verses.join("\n")
// }
// fn bottles(n: i32) -> &'static str {
//     match n {
//         1 => "bottle",
//         _ => "bottles"
//     }
// }