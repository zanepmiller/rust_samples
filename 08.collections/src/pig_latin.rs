// Module to convert arbitrary strings to pig latin.
// In reference to Rust book 8.3:
//  "Convert strings to pig latin. The first consonant of each word is moved to
//   the end of the word and ay is added, so first becomes irst-fay. Words that
//   start with a vowel have hay added to the end instead (apple becomes 
//   apple-hay). Keep in mind the details about UTF-8 encoding!"

const PUNCTUATION : &'static [&str] = &[":", ";",",", ".", "?"];
const VOWELS : &'static [&str] = &["a", "e", "i", "o", "u"];

pub fn pigify(input : &String) -> String {
    let mut ret_val = String::new();

    for word in input.split(" ") {
    }

    ret_val
}