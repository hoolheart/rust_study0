use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn guess_number() {
    //print!("Prepare a secret number: ");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("{}", secret_number);

    println!("Game on: guess the number!");

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read a line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number, try again!");
                continue;
            }
        };
        print!("Your guess is {}: ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn test_variables() {
    // let x : i32;
    // println!("Uninitialized x: {}", x);
    let x = 5;
    println!("Initialized x: {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("X in sub domain: {}", x);
    }
    println!("X in outer domain: {}", x);
    let mut x : i32;
    // println!("Uninitialized mutable x: {}", x);
    x = 42;
    println!("Mutable x: {}", x);
}

fn test_const() {
    const x : i32 = 7;
    const y : i32 = 9;
    println!("Two consts {} and {}", x, y);
}

fn main() {
    // guess_number();

    test_variables();
    test_const();
}
