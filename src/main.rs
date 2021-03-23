mod machine;
mod player;
mod selector;

fn main() {
    let player_guess = player::guess();
    let machine_guess = machine::guess();
    selector::get_decison(&player_guess, &machine_guess);
}