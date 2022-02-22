use rust_study0::*;

fn main() {
    test_arguments();

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

    test_variables();
    test_const();

    test_types();

    test_ownership();

    test_struct();

    test_enum();

    test_vector();
    test_string();
    test_hash();

    test_generic();
}
