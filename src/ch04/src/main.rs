fn main() {
    let text = " how old are you";
    println!("{}", first_word(text));
    let a = [1, 2, 3, 4, 5];
    let sli = &a[..2];
    println!("{:?}", sli)
}

fn first_word(text: &str) -> &str {
    if text.len() == 0 {
        return "";
    }
    let t = text.trim();
    for (i, word) in t.trim().as_bytes().iter().enumerate() {
        if *word == b' ' {
            return &t[..i];
        }
    }

    return t;
}
