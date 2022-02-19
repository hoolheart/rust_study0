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

fn test_ownership() {
    let mut s = String::from("hello");
    println!("{}", s);
    print_variable_info(&s);

    s.push_str(", world!");
    println!("{}", s);
    print_variable_info(&s);

    let sentence = &s[..];
    println!("Slice: {}, len: {}", sentence, sentence.len());
    let word0 = &s[0..5];
    println!("Slice: {}, len: {}", word0, word0.len());
    let word1 = &s[7..12];
    println!("Slice: {}, len: {}", word1, word1.len());
}

fn test_struct() {
    struct User {
        active : bool,
        email: String,
        name: String,
        sign_in_count : u64,
    }

    //let user0 = User {}; //forbidden default construction
    //print_variable_info(&user0);

    let user1 = User {
        active: true,
        email: String::from("abc@d.com"),
        name: String::from("abc"),
        sign_in_count: 1,
    };
    print_variable_info(&user1);

    let user2 = user1;
    //print_variable_info(&user1);// has moved
    print_variable_info(&user2);

    let user3 = &user2;
    println!("User 2 name {}, email {}, active {}, count {}", user2.name, user2.email, user2.active, user2.sign_in_count);
    println!("User 3 name {}, email {}, active {}, count {}", user3.name, user3.email, user3.active, user3.sign_in_count);

    let user4 = User {
        name: String::from("xyz"),
        email: String::from("xyz@q.com"),
        ..user2
    };
    println!("User 2 name {}, email {}, active {}, count {}", user2.name, user2.email, user2.active, user2.sign_in_count);
    println!("User 4 name {}, email {}, active {}, count {}", user4.name, user4.email, user4.active, user4.sign_in_count);

    let mut user5 = User {
        active: false,
        sign_in_count: 276,
        ..user4
    };
    //println!("User 4 name {}, email {}", user4.name, user4.email); // moved
    println!("User 5 name {}, email {}, active {}, count {}", user5.name, user5.email, user5.active, user5.sign_in_count);

    struct Color(i32, i32, i32);
    struct Point3D(i32, i32, i32);

    print_variable_info(&Color(0, 0, 0));
    print_variable_info(&Point3D(0, 0, 0));

    struct EmptyStruct {}

    let subject = EmptyStruct{};
    print_variable_info(&subject);
}

fn main() {
    //let chances : u32 = 3;
    //match guess_number(true, chances).1 {
    //    true => println!("Succeed to guess the number"),
    //    false => println!("Failed to guess the number in {} times", chances)
    //}
    //println!();

    //let (answer, result) = guess_number(false, chances);
    //if result {
    //    println!("Succeed to guess the number");
    //} else {
    //    println!("Failed to guess {} in {} times", answer, chances);
    //}

    // test_variables();
    // test_const();

    // test_types();

    //test_ownership();

    test_struct();
}
