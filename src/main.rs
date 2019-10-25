use std::io;
use rand;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guessing game");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess...");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("Input is {}", guess);
        let guess: i32 = guess.trim().parse().expect("Type a number");
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
