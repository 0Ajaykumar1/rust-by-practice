// Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
// Modify `assert_eq!` to make it work

fn main(){
    let x = 5; // default type infer i32
    assert_eq!("i32".to_string(), type_of(&x)); //assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}