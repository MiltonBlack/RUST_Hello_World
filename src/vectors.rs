pub fn run_vectors(){
    let mut numberz: Vec<i8> = vec![1,7,7];
    println!("{:?}", numberz);
    numberz.push(0);
    numberz.push(1);
    println!("{:?}", numberz);
    numberz.pop();
    println!("{:?}", numberz);
    for x in numberz.iter(){
        println!("Number: {}", x);
    }
    for x in numberz.iter_mut(){
        *x *= 2;
    }
    println!("Number: {:?}", numberz);
}