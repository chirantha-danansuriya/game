use std::io::stdin;

pub fn guess() -> u32 {
    println!("Please guess the number bitween 1 and 100: ");
    let mut user_input: String = String::new();
    stdin().read_line(&mut user_input).expect("Error");
    let user_input: u32 = user_input.trim().parse().expect("Please enter a number");
    user_input
}