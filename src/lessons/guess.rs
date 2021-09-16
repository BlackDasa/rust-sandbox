use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guess(){

    let number_to_guess = rand::thread_rng().gen_range(1..101);

    println!("Welcome to Guess a Number Game (Rustacean Edition).");
    println!("Please, enter a number.");
    
    loop{
        let mut number_guessed_from_user = String::new();
        io::stdin()
        .read_line(&mut number_guessed_from_user)
        .expect("Failed to read line");

        let number_guessed_from_user: u32 = match number_guessed_from_user.trim().parse(){
                                                Ok(number) => number,
                                                Err(_) => continue,
                                            };
        
        println!("You guessed: {}", number_guessed_from_user);
        match number_guessed_from_user.cmp(&number_to_guess){
            Ordering::Greater => println!("Number too big!!!"),
            Ordering::Less=> println!("Number too small!!!"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        };
    }

}