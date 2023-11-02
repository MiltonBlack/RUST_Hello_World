pub fn run_strings(){
    // Default 'str' with fixed-length string in memory
    let game = "grand theft";
    let mut pc = String::from("ASUS ROG");
    println!("{}, {}, {}", pc, game, pc.len() + game.len());
    pc.push('s');
    pc.push_str(" AMD Ryzen");
    println!("{}, {}, {}", pc, game, pc.len() + game.len());

    let mut a = String::with_capacity(10);
    a.push('m');
    a.push('y');
    println!("{}", a);
    println!("{}", a.capacity());
}