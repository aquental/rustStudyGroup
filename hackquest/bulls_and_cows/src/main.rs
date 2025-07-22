use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Bulls and Cows");
    let secret_number = rand::thread_rng().gen_range(1..11);
    let mut attempts = 0;
    loop {
        println!("Please input a number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Oops! Something goes wrong");
        let guess = guess.trim();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };
        if guess < 1 || guess > 10 {
            println!("Please input a number between 1 and 10");
            continue;
        }
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("Congratulation you're right!");
                println!("You took {} attempts", attempts);
                break;
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!");
                if attempts > 5 {
                    println!("You have tried {} times", attempts);
                }
            }
            std::cmp::Ordering::Less => {
                println!("Too small!");
                if attempts > 5 {
                    println!("You have tried {} times", attempts);
                }
            }
        }
        attempts += 1;
    }
}
