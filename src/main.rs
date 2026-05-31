
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main(){
    println!("guess the number");

    // for setting a range we use .gen_range(1..=100)
    let secret_number = rand::thread_rng().gen_range(1..=100);

   

    loop{
        println!("input yo guess");
    
    

     let mut guess = String::new();

         io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // : u32  says that string is being converted into numerical.
        // .trim() cuts all the spaces. .parse() lets you change string to numerical

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

        println!("You guessed: {guess}");


    match guess.cmp(&secret_number){
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => {println!("you win");
        break;
    }
    }
    }

}
