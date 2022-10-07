fn main() {
    let money = String::from("money is great");

    let first = first_word(&money);

    println!("First word of \"{}\" = \"{}\"", money, first);

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];

    println!("\"{}\" \"{}\"", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
