pub fn run_cli(){
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    // println!("{:?}", args);
    // println!("{}", command);
    
    if command == "dice" {
        println!("Starting sequence on the command line interface...");
        println!("Program in session");
        println!("you are now working in the black dice integrated environment!!");
    } else if command == "black" {
        println!("Program running in session with the '{}' flag", command);
    } else {
        println!("Program Execution Failed and Terminated Sequence!!! Unknown command '{}', supported Args are: dice, black", command);
    }
}