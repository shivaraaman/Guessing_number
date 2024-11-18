use std::io;
use rand::thread_rng;
use rand::Rng;

fn main() {
    let random_number = thread_rng().gen_range(1..101);
    loop {
        println!("Guess number between 1 and 100:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: i32 = input.trim().parse().unwrap();
        if input == random_number {
            println!("Congrats You Win!!!");
            break;
        } else if input < random_number {
            println!("Your Guess is Too low! \n Try again: {}", input);
        } else {
            println!("Your Guess is Too high!\n Try again: {}", input);
        }
    }
}


