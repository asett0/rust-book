fn main() {
    let phrase = "the sky is blue";
    let pig_phrase = phrase
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&word| pig(word))
        .collect::<Vec<String>>()
        .join(" ");

    println!(
        "The pharse \"{}\" in pig latin is \"{}\" ",
        phrase, pig_phrase
    );
}

fn pig(word: &str) -> String {
    if word != "" && word.chars().all(|c| ('a'..='z').contains(&c)) {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        if vowels.contains(&word.chars().nth(0).unwrap()) {
            format!("{}-hay", word)
        } else {
            format!(
                "{}-{}ay",
                word.chars().skip(1).collect::<String>(),
                word.chars().nth(0).unwrap()
            )
        }
    } else {
        if word == "" {
            panic!("Empty string provided to pig function.")
        } else {
            panic!(
                "String with non-lowercase english alphabet letters provided pig function: {}",
                word
            )
        }
    }
}
