use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Please input you Guess: ");

    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");

    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("Secret number is: {}", secret_number);

    let guess: u32 = guess.trim().parse().expect("Pleae type number");

    println!("Your guess is: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Equal => println!("Equal!"),
        Ordering::Greater => println!("Too Greater!")
    }
}
