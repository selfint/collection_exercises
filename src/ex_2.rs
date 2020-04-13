/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/
pub fn piggify(word: &String) -> String {
    // generate new word
    let mut piggified_word = String::new();

    // check if the first char exists, if not return the word (it is empty)
    let mut char_iterator = word.chars();
    match char_iterator.next() {
        Some(c) => {
            if "aeiou".contains(c) {
                piggified_word += "-hay";
            } else {
                piggified_word += &("-".to_string() + &c.to_lowercase().to_string() + "ay");
            }
        }
        None => return word.to_string() + "ay",
    };

    // copy the rest of the word into piggified word
    for consonant in char_iterator.rev() {
        if consonant != '\n' {
            piggified_word.insert(0, consonant);
        }
    }

    piggified_word
}
