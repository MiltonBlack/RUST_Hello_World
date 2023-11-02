pub fn run_loops(){
    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 15 {
            break;
        };
    };

    // While loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        };
        count += 1;
    };

    // for loop
    for x in 0..20 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        };
    };
}