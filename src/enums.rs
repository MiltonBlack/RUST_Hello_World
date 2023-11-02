// Enums are types that have a few definite value.

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_player(m: Movement){
    // Perform action depending on the info
    match m {
        Movement::Up => println!("Player Up"),
        Movement::Down => println!("Player Down"),
        Movement::Left => println!("Player Left"),
        Movement::Right => println!("Player Right")
    }
}
pub fn run_enum(){
    let player_one = Movement::Left;
    let player_two = Movement::Right;
    let player_three = Movement::Up;
    let player_four = Movement::Down;

    move_player(player_one);
    move_player(player_two);
    move_player(player_three);
    move_player(player_four);
}