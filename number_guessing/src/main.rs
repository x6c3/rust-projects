use std:: io;
use rand:: Rng;
use std:: cmp:: Ordering;
use colored:: *;

fn main () {
    println!("Guessing Game!!");
    loop {
        println!("Enter a number ");

        let mut guess = String:: new();
        let random = rand:: thread_rng().gen_range(1..101);

        io:: stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("The secret number is: {}", random);

        match guess.cmp(&random) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You guessed the right number".green());
                break;
            },
            Ordering::Greater => println!("{}", "Too big".yellow()),
        }
    }
}