use rand::Rng;
use std::io;

fn main() {
    println!("RANDOM NUMBER GAME");

    let secret_number = rand::rng().random_range(1..=100);
    let mut health = 10;

    println!("You have {health} guesses");

    while true {
        println!("Enter your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Failed to parse: guess is not a number");

        println!("Your guess: {guess}");

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            break;
        }

        health -= 1;

        if health <= 0 {
            println!("Game over");
            println!("Secret number: {secret_number}");
            return;
        }
        println!("Your current guess left: {health}");
    }

    println!("You win!");

    println!("Secret number: {secret_number}");
}
