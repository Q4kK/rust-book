use rand::Rng;
use std::io;

fn main() {
    loop {
        let num: i32 = rand::thread_rng().gen_range(1..=100);

        println!("The secret number is {num}");

        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        while num.to_string() != guess {
            println!("Wrong guess.");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
        }

        println!("You guessed right!");
        break;
    }
}
