pub fn run_vars(){
    let name: &str = "Milton";
    // Rust Data Types are Immutable by Default, add "mut" to make them mutable.
    let mut age: i32 = 23;
    println!("My name is {} and i'm {} years old", name, age);
    age = 25;
    println!("My name is {} and i'm {} years old", name, age);

    // Define a Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign Multiple Variables
    let ( my_car, my_headset, fav_number) = ("Crosstour", "Beats by Dr.Dre", 4);
    println!("I drive a Honda {my_car} and use the {my_headset} because my lucky number is {fav_number}");
}