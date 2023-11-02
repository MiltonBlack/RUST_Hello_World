pub fn run_condition(){
    let age: i32 = 23;
    let check_id: bool = false;
    let knows_someone: bool = true;

    if age >= 21 && check_id || knows_someone {
        println!("What Movie would you like to watch?");
    } else if age >= 18 {
        println!("Parental Guidiance is Advised. Content Rated '20' !!!");
    } else {
        println!("Parental Guidiance is Advised. Content Rated '22' !!!");
    };

    let of_age = if age >= 21 { true } else { false };
    println!("Adult?: {}", of_age);
}