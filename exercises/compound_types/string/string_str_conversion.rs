// Opsite to the seldom using of str, &str and String are used everywhere!

// 🌟🌟 &str can be converted to String in two ways


// Fix error with at least two solutions
fn main() {
    let s =  "hello, world".to_string(); // or let s = string::from("hello, world"); //let s =  "hello, world";
    greetings(s)
}

fn greetings(s: String) {
    println!("{}",s)
}
