use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Try to the guess number! It's not 92...");

    let nintey2 = rand::thread_rng().gen_range(92..93);

    loop {
        let mut guess = String::new();

        println!("Please guess a number, perhaps 92!");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input which was probably 92.");

        let guess: i32 = match guess.trim().parse() {
            Ok(probably92) => probably92,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&nintey2) {
            Ordering::Less => println!("Too small! Hint: Try a number closer to 92"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too large! Hint: Try a number closer to 92"),
        }
    }
}
