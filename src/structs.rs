
// Traditional Stuct
// used to create custom data types
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tupple Struct
struct Ink(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // construct a person
    fn new(first: &str, last: &str) -> Person {
        Person { 
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
    // method of a Full Name
    // &self is same as this in javasctipt, also means the Person impl
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set first name mutable
    fn set_first_name(&mut self, first: &str) {
        self.first_name = first.to_string();
    } 

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run_structs(){
    // similar to classes in java, c#, javascript.
    let mut paint = Color {
        red: 255,
        green: 0,
        blue:100
    };

    println!("Struct Color: {} {} {}", paint.red, paint.green, paint.blue);
    paint.green = 175;
    println!("Struct Color: {} {} {}", paint.red, paint.green, paint.blue);
    
    let mut brush = Ink(23, 233, 65);
    println!("Tuple Struct Color: {} {} {}", brush.0, brush.1, brush.2);
    brush.0 = 177;
    println!("Tuple Struct Color: {} {} {}", brush.0, brush.1, brush.2);

    let mut identity = Person::new("Milton", "Black");
    println!("Identity: {} {}", identity.first_name, identity.last_name);
    identity.set_first_name("Sonia");
    println!("Full Identity: {}", identity.full_name());
    println!("Full Identity: {:?}", identity.to_tuple());
}