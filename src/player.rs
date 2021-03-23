use std::io::stdin;

pub fn guess() -> u32 {
    println!("Please guess the number between 1 and 100: ");
    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).expect("Error");
    let user_input: u32 = user_input.trim().parse().expect("Please enter a number");
    validate_user_input(user_input)
}

fn validate_user_input(user_input: u32) -> u32 {
    if user_input > 100 {
        panic!("Value should be between 1 and 100");
    } 
    user_input
}