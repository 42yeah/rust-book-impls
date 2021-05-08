fn main() {
    let mut input = String::new();
    println!("What do you want to pig-latinify? ");
    std::io::stdin().read_line(&mut input).unwrap();

    let mut ret = String::new();
    for word in input.split_whitespace() {
        let mut new_word = String::new();
        let remaining = &word[1..];
        if let Some(first) = word.chars().nth(0) {
            match first {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    new_word.push(first);
                    new_word += remaining;
                    new_word += "-";
                    new_word += "h";
                    new_word.push(first);
                    new_word += "y";
                },
                _ => {
                    new_word += remaining;
                    new_word += "-";
                    new_word.push(first);
                    new_word += "ay";
                }
            }
            ret += &new_word;
            ret += " ";
        } else {
            continue;
        }
    }
    println!("{}", ret);
}
