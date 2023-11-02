use std::io;

pub fn run_type_casting(){
    let w = 5_000_000_i64;
    let x = 10 as i64;
    // let y: = 20_i16;
    let z = 30i32;

    let v = w / (z as i64);

    let mut input = String::new();
    io::stdin().read_line
    println!("{}", v);
}