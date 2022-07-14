// The type of array is [T; Length], as you can see, array's length is part of their type signature. 
// So their length must be known at compile time.


fn main() {
    // Fill the blank with proper array type
    let arr: [i32;5] = [1, 2, 3, 4, 5]; //let arr: __ = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5); //assert!(arr.len() == 4);

    println!("Success!");
}
