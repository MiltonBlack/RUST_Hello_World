pub fn run(){
    // Print to Console
    println!("Hello from the Print.rs file...");
    println!("Number: {}", 1);
    // Basic Formatting
    println!("{} is from {}", "Milton", "The Moon");
    // Positional Arguments
    println!("{0} is a {1}  and a {2}. {0} is also a Blockchain Developer in {3}!", "Milton", "Fullstack Web Developer", "Mobile Developer", "Solana");
    // Named Arguments
    println!("{name} loves {activity}", name = "Milton", activity = "Coding");
    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 20, 20, 20);
    // Basic Math
    println!("25 + 45 = {}", 25 + 45);
}