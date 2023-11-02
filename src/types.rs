pub fn run_types(){
    // Prints the Maximum Value of Data Type
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    let is_active: bool = false;
    println!("{is_active}");

    // Get Boolean from an Expression
    let is_greater = 20 < 5;
    println!("{is_greater}");

    // Define Character
    let letter = 'b';
    let emoji = '\u{1F621}';
    println!("{letter} {emoji}");
}