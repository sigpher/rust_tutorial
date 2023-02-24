use std::io;

use rand::Rng;

fn main() {
    guessing_game()
}

pub fn guessing_game() {
    println!("Guess the number");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("please input number");
        let guess = guess.trim().parse::<u8>().unwrap();
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too Big!"),
            std::cmp::Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
