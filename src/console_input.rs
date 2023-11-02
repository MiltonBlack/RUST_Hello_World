use std::io;

pub fn run_console_input(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to Read Input!!!");
    println!("{}", input);
}