pub fn run_funct(){
    greet("Good Morning", "Sonia");

    // Bind Function Values To Variables
    let get_sum: i32 = add_numbers(5, 6);
    println!("Sum = {}", get_sum);

    // Closure
    let n3 = 4;
    let divide_num = |n1: i32, n2:i32| (n1 / n2) + n3;
    println!("Closure Sum: {}", divide_num(100, 2));
}

fn greet(greet: &str, name: &str){
    println!("{} {}, it's nice meeting you.", greet, name);
}

// Function with a Return
fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2
}