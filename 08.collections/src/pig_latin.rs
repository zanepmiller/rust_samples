// Module to convert arbitrary strings to pig latin.
// In reference to Rust book 8.3:
//  "Convert strings to pig latin. The first consonant of each word is moved to
//   the end of the word and ay is added, so first becomes irst-fay. Words that
//   start with a vowel have hay added to the end instead (apple becomes 
//   apple-hay). Keep in mind the details about UTF-8 encoding!"

const PUNCTUATION : &'static [char] = &[':', ';', ',', '.', '?'];
const VOWELS : &'static [char] = &['a', 'e', 'i', 'o', 'u'];

pub fn pigify(input : &String) -> String {
    let mut ret_val = String::new();

    for line in input.lines() {
        for word in line.split(" ") {
            if word.len() < 1 {continue};
            let first = match word.chars().next() {
                Some(i) => i,
                None => '\0',
            };
            let last = match word.chars().last() {
                Some(i) => i,
                None => '\0',
            };
            
            let mut suffix = String::from("");
            match first.is_ascii_alphabetic() {
                true => {
                    match VOWELS.contains(&first.to_ascii_lowercase()) {
                        true => {
                            suffix.push_str("-hay");
                            ret_val.push_str(&word[..word.len()-1]);
                        },
                        false => {
                            suffix.push_str(&format!("-{first}ay"));
                            ret_val.push_str(&word[1..word.len()-1]);
                        },
                    }
                },
                false => ret_val.push_str(&word[..word.len()-1]),
            };

            match PUNCTUATION.contains(&last) {
                true => suffix.push(last),
                false => ret_val.push(last),
            };
            
            ret_val = ret_val + &suffix + &" ";
        }
        ret_val.push('\n');
    }

    ret_val
}