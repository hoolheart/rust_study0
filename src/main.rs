use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::mem;

// game to guess a random number
fn guess_number(show_answer : bool, max_cnt : u32) -> (u32, bool) {
    let secret_number = rand::thread_rng().gen_range(1..101); // generate a random number to guess
    if show_answer {
        println!("Prepare a secret number: {}", secret_number);
    }

    println!("Game on: guess the number!");

    let mut count = 0;
    let result = loop {
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read a line"); // get a guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number, try again!"); // input is not number, require user to try again
                continue;
            }
        };
        print!("Your guess is {}: ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break true; // guess succeed
            }
        }

        count += 1;
        if (max_cnt > 0) && (count >= max_cnt) {
            break false; // failed in limited chances
        }
    };

    (secret_number, result)
}

fn test_variables() {
    // let x : i32;
    // println!("Uninitialized x: {}", x); //Error: uninitialized
    let x = 5;
    println!("Initialized x: {}", x);
    let x = x + 1;
    {
        let x = x * 2;
        println!("X in sub domain: {}", x);
    }
    println!("X in outer domain: {}", x);
    let mut x : i32;
    // println!("Uninitialized mutable x: {}", x); //Error: uninitialized
    x = 42;
    println!("Mutable x: {}", x);
    x = 16;
    println!("Change mutable x into: {}", x);
}

fn test_const() {
    const X : i32 = 7;
    const Y : i32 = 9;
    println!("Two consts {} and {}", X, Y);
}

fn print_variable_info<T> (v: &T) {
    println!("type {} size {}", std::any::type_name::<T>(), mem::size_of_val(v));
}

fn test_types() {
    let a = 234u8;//interger types
    print_variable_info(&a);
    print_variable_info(&234);
    let a : i128 = 998_222;
    print_variable_info(&a);
    print_variable_info(&b'0');
    print_variable_info(&0xffff0778u32);
    print_variable_info(&0x798);

    print_variable_info(&0.665);//float types
    print_variable_info(&0.5432f32);

    print_variable_info(&true);//bool
    let a = false;
    print_variable_info(&a);

    print_variable_info(&'这');//char
    print_variable_info(&'🤦');

    let a = (500, 7.8, '&');//tuple
    print_variable_info(&a);
    print_variable_info(&a.0);
    let (x, y, z) = a;
    print_variable_info(&x);
    print_variable_info(&y);
    print_variable_info(&z);

    let a = [7, 8, 9, 10];//array
    print_variable_info(&a);
    print_variable_info(&a[0]);

    let a : [i16; 5] = [1, 2, 3, 4, 5];
    print_variable_info(&a);
    print_variable_info(&a[0]);

    let a = [5.5; 3];
    print_variable_info(&a);
}

fn main() {
    let chances : u32 = 3;
    match guess_number(true, chances).1 {
        true => println!("Succeed to guess the number"),
        false => println!("Failed to guess the number in {} times", chances)
    }
    println!();

    let (answer, result) = guess_number(false, chances);
    if result {
        println!("Succeed to guess the number");
    } else {
        println!("Failed to guess {} in {} times", answer, chances);
    }

    // test_variables();
    // test_const();

    // test_types();
}
