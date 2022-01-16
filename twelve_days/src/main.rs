fn main() {
    song();
}

fn day_lyrics(s: &str) -> Vec<String> {
    vec![
        String::from(""),
        format!("On the {} day of Christmas", s),
        String::from("My true love sent to me"),
    ]
}

fn verse<'a>(day: &str, present: &'a str, present_lyrics: &mut Vec<&'a str>) {
    present_lyrics.push(present);

    for lyric in day_lyrics(day) {
        println!("{}", lyric);
    }

    for &lyric in present_lyrics.iter().rev() {
        println!("{}", lyric);
    }
}

fn song() {
    let mut present_lyrics: Vec<&str> = Vec::new();

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let presents = [
        "A partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for (&day, present) in days.iter().zip(presents) {
        verse(day, present, &mut present_lyrics);

        if day == "first" {
            present_lyrics.pop();
            present_lyrics.push("And a partridge in a pear tree");
        }
    }

    println!("And a partridge in a pear tree!");
}
