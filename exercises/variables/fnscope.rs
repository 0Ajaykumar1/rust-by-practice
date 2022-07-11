// A scope is the range within the program for which the item is valid.
// Fix the error with the use of define_x

fn main(){
    let x = define_x();
    println!("{}, world", x);
}

fn define_x() -> String {
    let x = "hello".to_string();
    x
}