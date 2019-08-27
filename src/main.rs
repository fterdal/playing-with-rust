use std::io;
use std::cmp::Ordering;
extern crate rand;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // let secret_number = 50;
    // let mut guess = String::new();
    loop {
        let mut guess = String::new();
        println!("Input your guess:");
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = guess.trim().to_string().parse().unwrap();
        println!("You guessed {}", guess);
        println!("The answer is {}", secret_number);
        println!("{:?}", guess.cmp(&secret_number));
        // match guess {

        // }
        if guess == secret_number {
            println!("You win!");
            break;
        }
    }
}
