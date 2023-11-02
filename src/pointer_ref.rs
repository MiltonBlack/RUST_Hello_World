pub fn run_pointer_ref(){
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Array Values: {:?}", (arr1, arr2));
    
    // non-primitive array
    // if you assign another variable to a piece of data, the first variable will no longer hold that value. You will require a reference (&) to point to the resource.
    
    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;
    println!("Vector Array Values: {:?}", (&vec1, vec2));
}