use std::io;

fn main() {
    let mut correct_num = 15;
    let mut guess = String::new();
    println!("Input your guess:");
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed {}", guess);
    loop {
        println!("{}", correct_num);
        correct_num -= 1;
        if correct_num <= 0 {
            break
        }
    }
}
