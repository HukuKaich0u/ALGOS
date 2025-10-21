fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("Hello world");

    let word = second_word(&s);

    (&mut s).clear();

    println!("The second word is: {}", word)
}
