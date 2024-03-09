fn add (a: i32, b: i32) -> i32 {
    a + b
}

fn sub (a: i32, b: i32) -> i32 {
    a - b
}


fn div (a: f32, b: f32) -> f32 {
    if b == 0.0 {
        -1.0
    } else {
        a / b
    }
}

fn mul (a: i32, b: i32) -> i32 {
    a * b
}

use std:: io;

fn main () {
    let mut choice = String:: new ();
    let mut a = String:: new ();
    let mut b = String:: new ();


    println!("Calculator");
    println!("Enter a: ");
    io:: stdin()
        .read_line(&mut a)
        .expect("Someting went wrong");
    let a: i32 = a.trim().parse().expect("Please enter a number");

    println!("Enter b: ");
    io:: stdin()
        .read_line(&mut b)
        .expect("Something went wrong");
    let b: i32 = b.trim().parse().expect("Please enter a number");

    println!("Enter your choice ");
    io:: stdin ()
        .read_line(&mut choice)
        .expect("Someting went wrong");
    let choice = choice.trim();


    if choice == "+" {
        let sum = add (a, b);
        println!("{}", sum);
    } 

    if choice == "-" {
        let sub = sub (a, b);
        println!("{}", sub);
    }

    if choice == "*" {
        let mul = mul (a, b);
        println!("{}", mul);
    }

    if choice == "/" {
        let div = div (a as f32, b as f32);
        println!("{}", div);
    }

}
