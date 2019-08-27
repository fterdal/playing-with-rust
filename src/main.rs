fn main() {
    let mut correct_num = 75;
    loop {
        println!("{}", correct_num);
        correct_num -= 1;
        if correct_num <= 0 {
            break
        }
    }
}
