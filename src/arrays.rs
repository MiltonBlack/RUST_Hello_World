use std::mem;

pub fn run_arrays(){
    let mut numberz: [i32; 11] = [0,9,0,3,7,2,8,9,1,9,2];
    println!("{:?}", numberz);
    numberz[1] = 8;
    println!("Single digit: {}", numberz[0]);
    println!("Full Array: {:?}", numberz);
    println!("Total Length of Array: {}", numberz.len());
    println!("This Array occupies {} bytes", mem::size_of_val(&numberz));
    let slice: &[i32] = &numberz[0..3];
    println!("slice: {:?}", slice);
}