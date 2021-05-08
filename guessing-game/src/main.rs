use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..100);

    loop {
        let mut guess = String::new();
        println!("Input your guess:");
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You have guessed: {}", guess);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Equal => {
                println!("That's right!");
                break;
            }
            Ordering::Greater => {
                println!("Too big!");
            }
        }
    }
}
