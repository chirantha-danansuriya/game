mod machine;
mod player;
mod selector;

fn main() {
    let machine_guess = machine::guess();
    loop {
        let player_guess = player::guess();
        let win = selector::get_decison(&player_guess, &machine_guess);

        if win {
            break;
        }
    }  
}