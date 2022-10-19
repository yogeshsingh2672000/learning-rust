use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    
    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("Secret number is: {}", secret_number);
    
    loop {
        println!("Please input you Guess: ");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        // let guess: u32 = guess.trim().parse().expect("Pleae type number");
        println!("Your guess is: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too Big!".red()),
        }
    }
    
    
    

}
