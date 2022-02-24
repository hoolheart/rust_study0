use rust_study0::*;

trait TestCase {
    fn invoke(&self);
}

struct TestFunction<T> where T: Fn(), {
    foo: T,
    permit: bool,
}

impl<T> TestCase for TestFunction<T> where T: Fn(), {
    fn invoke(&self) {
        if self.permit {
            (self.foo)();
        }
    }
}

impl<T> TestFunction<T> where T: Fn(), {
    fn valid(foo: T) -> TestFunction<T> {
        TestFunction {
            foo: foo,
            permit: true,
        }
    }
    fn invalid(foo: T) -> TestFunction<T> {
        TestFunction {
            foo: foo,
            permit: false,
        }
    }
}

fn main() {
    let chances = 10;

    let cases: Vec<Box<dyn TestCase>> = vec![
        Box::new(TestFunction::valid(||{
            match guess_number(true, chances).1 {
                true => println!("Succeed to guess the number"),
                false => println!("Failed to guess the number in {} times", chances)
            }
        })),
        Box::new(TestFunction::invalid(||{
            let (answer, result) = guess_number(false, chances);
            if result {
               println!("Succeed to guess the number");
            } else {
               println!("Failed to guess {} in {} times", answer, chances);
            }
        })),
        Box::new(TestFunction::invalid(||{
            test_variables();
            test_const();
        })),
        Box::new(TestFunction::invalid(|| {test_types();})),
        Box::new(TestFunction::invalid(|| {test_ownership();})),
        Box::new(TestFunction::invalid(|| {test_struct();})),
        Box::new(TestFunction::invalid(|| {test_enum();})),
        Box::new(TestFunction::invalid(|| {
            test_vector();
            test_string();
            test_hash();
        })),
        Box::new(TestFunction::invalid(|| {test_generic();})),
        Box::new(TestFunction::valid(|| {
            let args = test_arguments(); // get argument list
            let file_name = parse_arg(&args); // get file name from arguments
            if let Err(e) = test_file(&file_name) {
                eprintln!("{:?}", e);
            }
        })),
        Box::new(TestFunction::valid(|| {test_closure();})),
    ];

    for case in cases {
        case.invoke();
    }
}
