use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret = rand::thread_rng().gen_range(1..=5);
    loop {
        println!("Enter a number!");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Error");
    
        // let guess: u32 = guess.trim().parse().expect("Use a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("Equal!");
                println!("Well done!");
                break;
            }
        }
        
    }

}
