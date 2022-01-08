// fn main() {
//     song();
// }

// fn day_lyrics(s: &str) -> Vec<String> {
//     vec![
//         String::from(""),
//         format!("On the {} day of Christmas", s),
//         String::from("My true love sent to me"),
//     ]
// }

// fn verse<'a>(day: &str, present: &'a str, present_lyrics: &'a mut Vec<&'a str>) {
//     present_lyrics.push(present);

//     for lyric in day_lyrics(day) {
//         println!("{}", lyric);
//     }

//     for &lyric in present_lyrics.iter().rev() {
//         println!("{}", lyric);
//     }
// }

// fn song() {
//     let mut present_lyrics: Vec<&str> = Vec::new();

//     verse("first", "A partridge in a pear tree", &mut present_lyrics);

//     // ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"]

//     // present_lyrics.push("A partridge in a pear tree");

//     // for lyric in day_lyrics("first") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     present_lyrics.pop();
//     present_lyrics.push("And a partridge in a pear tree");

//     verse(
//         "second",
//         &String::from("Two turtle-doves"),
//         &mut present_lyrics,
//     );
//     // present_lyrics.push("Two turtle-doves");

//     // for lyric in day_lyrics("second") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Three French hens");

//     // for lyric in day_lyrics("third") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Four calling birds");

//     // for lyric in day_lyrics("fourth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Five golden rings (five golden rings)");

//     // for lyric in day_lyrics("fifth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Six geese a laying");

//     // for lyric in day_lyrics("sixth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Seven swans a swimming");

//     // for lyric in day_lyrics("seventh") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Eight maids a milking");

//     // for lyric in day_lyrics("eigth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Nine ladies dancing");

//     // for lyric in day_lyrics("ninth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Ten lords a-leaping");

//     // for lyric in day_lyrics("tenth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Eleven pipers piping");

//     // for lyric in day_lyrics("eleventh") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     // present_lyrics.push("Twelve drummers drumming");

//     // for lyric in day_lyrics("twelfth") {
//     //     println!("{}", lyric);
//     // }

//     // for &lyric in present_lyrics.iter().rev() {
//     //     println!("{}", lyric);
//     // }

//     println!("And a partridge in a pear tree!");
// }
