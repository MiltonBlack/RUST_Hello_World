mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;
mod console_input;
mod type_casting;
// importing External Files

fn hello() {
    println!("Hi, I am now a Rustician!!!");
    vars::run_vars();
}
fn main() {
    let result = 10; //i32 by def
    // let _course:&str = "hello there";
    println!("Hello, world! {}", result);

    // call the hello function!!
    hello();

    // call the run function in the imported print.rs file
    print::run();
    types::run_types();
    strings::run_strings();
    tuples::run_tuples();
    arrays::run_arrays();
    vectors::run_vectors();
    conditionals::run_condition();
    loops::run_loops();
    functions::run_funct();
    pointer_ref::run_pointer_ref();
    structs::run_structs();
    enums::run_enum();
    cli::run_cli();
    console_input::run_console_input();
    type_casting::run_type_casting();
}
