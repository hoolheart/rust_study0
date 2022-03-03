use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::mem;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::error::Error;

// game to guess a random number
pub fn guess_number(show_answer : bool, max_cnt : u32) -> (u32, bool) {
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

pub fn test_variables() {
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

pub fn test_const() {
    const X : i32 = 7;
    const Y : i32 = 9;
    println!("Two consts {} and {}", X, Y);
}

pub fn print_variable_info<T> (v: &T) {
    println!("type {} size {}", std::any::type_name::<T>(), mem::size_of_val(v));
}

pub fn test_types() {
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

pub fn test_ownership() {
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

pub fn test_struct() {
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

pub fn test_enum() {
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

pub fn test_vector() {
    let v0: Vec<i32> = Vec::new();
    print_variable_info(&v0);

    let v1 = vec![768.0, 66.7, 98.1];
    print_variable_info(&v1);
    let mut i_v1 = 0;
    loop {
        match v1.get(i_v1) {
            Some(val) => println!("{}th value: {}", i_v1, val),
            None => {
                println!("V1 len: {}", i_v1);
                break
            }
        }
        i_v1 += 1;
    }

    for (i, &element) in v1.iter().enumerate() {
        println!("{}: {}", i, element);
    }

    let mut v2: Vec<i32> = Vec::new();
    println!("Vector 2 before edition: {:?}", v2);
    v2.push(67);
    v2.push(98);
    v2.push(234232);
    println!("Vector 2 after edition: {:?}", v2);

    let mut v3 = vec!['a', 'c', '5'];
    let a = &v3[0];
    let b = v3[1];
    println!("{}, {}", a, b);
    v3.push('@');
    // println!("{}", a); // modification breaks all references
    println!("{}", b);

    print!("[");
    for item in &v3 {
        print!("{}, ", item);
    }
    println!("]");
}

pub fn test_string() {
    let mut s0 = "你好, 这里是苏州wow!".to_string();
    println!("{} {:?} {}", s0, s0, s0.len());
    print_variable_info(&s0);

    s0.push_str("שָׁלוֹם");
    for ch in s0.chars() {
        print!("'{}' ", ch);
    }
    println!();

    for byte in s0.bytes() {
        print!("{} ", byte);
    }
    println!();
}

pub fn test_hash() {
    let mut devices = HashMap::new();
    let mut device_models = HashMap::new();

    #[derive(Debug)]
    enum AttrValue {
        I32(i32),
        I64(i64),
        U32(u32),
        U64(u64),
        Float(f64),
        Str(String),
    }

    impl Clone for AttrValue {
        fn clone(&self) -> AttrValue {
            match self {
                AttrValue::Str(s) => AttrValue::Str(s.clone()),
                AttrValue::I32(val) => AttrValue::I32(*val),
                AttrValue::I64(val) => AttrValue::I64(*val),
                AttrValue::U32(val) => AttrValue::U32(*val),
                AttrValue::U64(val) => AttrValue::U64(*val),
                AttrValue::Float(val) => AttrValue::Float(*val),
            }
        }
        fn clone_from(&mut self, source : &AttrValue) {
            match source {
                AttrValue::Str(s) => *self = AttrValue::Str(s.clone()),
                AttrValue::I32(val) => *self = AttrValue::I32(*val),
                AttrValue::I64(val) => *self = AttrValue::I64(*val),
                AttrValue::U32(val) => *self = AttrValue::U32(*val),
                AttrValue::U64(val) => *self = AttrValue::U64(*val),
                AttrValue::Float(val) => *self = AttrValue::Float(*val),
            }
        }
    }

    #[derive(Debug)]
    struct Device {
        addr: String,
        category: String,
        model: String,
        attributes: HashMap<String, AttrValue>,
    }

    impl Device {
        fn clone(&self, addr : &str) -> Device {
            Device {
                addr: addr.to_string(),
                category: self.category.clone(),
                model: self.model.clone(),
                attributes: self.attributes.clone(),
            }
        }
    }

    let models = [("aaa", "lt0"), ("bbb", "mw"), ("ccc", "lab"), ("ddd", "mw"), ("aaa", "lt")];
    let mut attributes = HashMap::new();
    attributes.insert(String::from("i32"), AttrValue::I32(56));
    attributes.insert(String::from("i64"), AttrValue::I64(56));
    attributes.insert(String::from("u32"), AttrValue::U32(56));
    attributes.insert(String::from("u64"), AttrValue::U64(56));
    attributes.insert(String::from("f64"), AttrValue::Float(56.1));
    attributes.insert(String::from("str"), AttrValue::Str(String::from("attribute")));
    for (m, c) in models {
        device_models.insert(m, Device {
            addr: String::new(),
            category: c.to_string(),
            model: m.to_string(),
            attributes: attributes.clone(),
        });
    }
    
    println!("{:#?}", device_models);

    let dev_info = [("aaa", "192.168.0.100"), ("bbb", "192.168.0.101"), ("bbb", "192.168.0.102"), ("bbb", "192.168.0.103"), ("ccc", "192.168.0.104")];
    for (m, a) in dev_info {
        //let model = String::from(m);
        if let Some(dev) = device_models.get(&m) {
            devices.insert(a.to_string(), dev.clone(a));
        }
    }

    println!("{:#?}", devices);
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut index = 0;
    for i in 1..list.len() {
        if list[i] > list[index] {
            index = i;
        }
    }
    &list[index]
}

pub fn test_generic() {
    let list0 = [34, 50, 25, 100, 65];
    println!("largest in list 0: {}", largest(&list0));

    let list1 = vec!['a', '7', '&', 'P'];
    println!("largest in list 1: {}", largest(&list1));

    let list2 = [5.5, 6.7, 11.9, 900.7];
    println!("largest in list 2: {}", largest(&list2));

    let list3 = ["oh", "lalala", "888"];
    println!("largest in list 3: {}", largest(&list3));
}

pub fn test_arguments() -> Vec<String> {
    let args: Vec<String> = dbg!(env::args()).collect();
    if args.len() > 0 {
        for (i, arg) in args.iter().enumerate() {
            println!("argument {}: {}", i, arg);
        }
    } else {
        println!("no argument input");
    }
    args
}

pub fn parse_arg(args: &[String]) -> String {
    if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("poem.txt")
    }
}

pub fn test_file(file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("Read file: {}", file_name);
    let contents = fs::read_to_string(file_name)?;
    println!("File contents:");
    println!("{}", contents);
    print_variable_info(&contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = if case_sensitive {query.to_string()} else {query.to_lowercase()};

    for line in contents.lines() {
        let line_str = if case_sensitive {line.to_string()} else {line.to_lowercase()};
        if line_str.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_with_iterator<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let query = if case_sensitive {query.to_string()} else {query.to_lowercase()};
    let f = |line: &&str| -> bool {
        let line_str = if case_sensitive {line.to_string()} else {line.to_lowercase()};
        line_str.contains(&query)
    };

    contents.lines().filter(f).collect()
}

pub fn test_closure() {
    let closure = |num: u32| -> u32 {
        num + 2
    };
    print_variable_info(&closure);

    println!("Plus 2 and 3 is {}", closure(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }

    #[test]
    fn multi_results() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:"], search("Rust", contents, true));
        assert_eq!(vec!["Rust:", "Trust me."], search("rUsT", contents, false));
    }

    #[test]
    fn multi_results_with_iterator() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:"], search_with_iterator("Rust", contents, true));
        assert_eq!(vec!["Rust:", "Trust me."], search_with_iterator("rUsT", contents, false));
    }
}
