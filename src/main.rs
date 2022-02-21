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
    #[derive(Debug)]
    struct User {
        active : bool,
        email: String,
        name: String,
        sign_in_count : u64,
    }

    impl User {
        fn register(name: &str, email: &str) -> User {
            User {
                active: true,
                email: String::from(email),
                name: String::from(name),
                sign_in_count: 1,
            }
        }
        fn valid(&self) -> bool {
            self.active && (self.email.len() > 0) && (self.name.len() > 0)
        }
        fn increase(&mut self) -> u64 {
            self.sign_in_count += 1;
            self.sign_in_count
        }
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
    println!("User 1 valid: {}", user1.valid());

    let user2 = dbg!(user1);
    //print_variable_info(&user1);// has moved
    print_variable_info(&user2);

    let user3 = &user2;
    println!("User 2 {:?}", user2);
    println!("User 3 {:?}", user3);

    let mut user4 = User {
        name: String::from("xyz"),
        email: String::from("xyz@q.com"),
        ..user2
    };
    println!("User 2 {:#?}", user2);
    println!("User 4 {:#?}", user4);

    println!("User 4 increase count of sign-in: {}", user4.increase());

    let user5 = User {
        active: false,
        // sign_in_count: 276,
        ..user4
    };
    //println!("User 4 name {}, email {}", user4.name, user4.email); // moved
    println!("User 5 name {}, email {}, active {}, count {}, valid {}", user5.name, user5.email, user5.active, user5.sign_in_count, user5.valid());

    let mut user6 = dbg!(User::register("lalala", ""));
    println!("User 6 valid: {}, increase count: {}", user6.valid(), user6.increase());

    struct Color(i32, i32, i32);
    struct Point3D(i32, i32, i32);

    print_variable_info(&Color(0, 0, 0));
    print_variable_info(&Point3D(0, 0, 0));

    struct EmptyStruct {}

    let subject = EmptyStruct{};
    print_variable_info(&subject);
}

fn test_enum() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(u16, u16, u16, u16, u16, u16, u16, u16),
    }

    impl IpAddr {
        fn from_v4(x0: u8, x1: u8, x2: u8, x3: u8) -> IpAddr {
            IpAddr::V4(x0, x1, x2, x3)
        }
        fn from_v6(x0: u16, x1: u16, x2: u16, x3: u16, x4: u16, x5: u16, x6: u16, x7: u16) -> IpAddr {
            IpAddr::V6(x0, x1, x2, x3, x4, x5, x6, x7)
        }
        fn version(&self) -> i32 {
            match self {
                IpAddr::V4(..) => 4,
                IpAddr::V6(..) => 6,
            }
        }
        fn to_string(&self) -> String {
            match self {
                IpAddr::V4(x0, x1, x2, x3) => format!("{}.{}.{}.{}", x0, x1, x2, x3),
                IpAddr::V6(x0, x1, x2, x3, x4, x5, x6, x7) => format!("{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}:{:04X}", x0, x1, x2, x3, x4, x5, x6, x7),
            }
        }
    }

    let addr0 = IpAddr::V4(127, 0, 0, 1);
    println!("{}", addr0.to_string());

    let addr1 = dbg!(IpAddr::V6(0xabcd, 0, 0, 0, 0x11bb, 0x77, 0, 1));
    println!("V{}, {}", addr1.version(), addr1.to_string());

    dbg!(IpAddr::from_v4(192, 168, 0, 1));
    dbg!(IpAddr::from_v6(192, 168, 0, 1, 0, 0, 0, 1));
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

    //test_variables();
    //test_const();

    //test_types();

    //test_ownership();

    //test_struct();

    test_enum();
}
