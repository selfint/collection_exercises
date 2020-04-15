/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/
pub fn piggify(word: &str) -> String {
    let mut piggified_word = String::new();

    // copy the rest of the word into piggified word
    for consonant in word.chars().skip(1) {
        // ignore newlines
        if consonant != '\n' {
            piggified_word.push(consonant);
        }
    }

    // check if the first char is a vowel or not
    match word.chars().next() {
        Some(c) => {
            if "aeiou".contains(c) {
                piggified_word.insert(0, c);
                piggified_word += "-hay";
            } else {
                piggified_word += &("-".to_string() + &c.to_lowercase().to_string() + "ay");
            }
        }
        None => return word.to_string() + "ay",
    };

    piggified_word
}
