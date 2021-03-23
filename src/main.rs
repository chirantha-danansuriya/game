mod machine;
mod player;
mod selector;

fn main() {
    let player_guess = player::guess();
    let machine_guess = machine::guess();

    println!("Player's guess: {}", player_guess);
    println!("Machine's guess: {}", machine_guess);

    selector::get_decison(&player_guess, &machine_guess);
}