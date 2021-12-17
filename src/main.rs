use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::mem;

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Prepare a secret number: {}", secret_number);

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
    guess_number();

    test_variables();
    test_const();

    test_types();
}
