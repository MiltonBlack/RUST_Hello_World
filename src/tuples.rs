// Tuples are group of values of different types.
// Max 12 elements
pub fn run_tuples(){
    let person: (&str, &str, i16) = ("Milton", "Crosstour", 2020);
    println!("{} bought a {} of {} Model", person.0, person.1, person.2);
}