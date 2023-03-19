fn main() {
    let mut m = String::from("Hello world!");
    
    let index = first_word(&m);
    println!("index: {}", index);

    
    let word = first_word_better(&m);
    // m.clear(); // should fail
    println!("First word: {}", word);


}

// sucks because manual, and word index not actually tied with the word
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_better(s: &String) -> &str { // use &str instead of &String, since you can pass reference of String as &str
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s;
}