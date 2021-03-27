pub fn get_decison(player_guess: &u32, machine_guess: &u32) -> bool{
    let mut win_status: bool = false;
    if player_guess > machine_guess {
        println!("Too High");
    } else if machine_guess > player_guess {
        println! ("Too Low");
    } else {
        println!("You Won");
        win_status = true;
    }
    win_status
}