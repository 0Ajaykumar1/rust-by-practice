// &String can be implicitly converted into &str.
// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // It works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref` 
    let word = first_word(&s);

    //s.clear(); // error!

    println!("the first word is: {}", word);
    s.clear();
}
fn first_word(s: &str) -> &str {
    &s[..1]
}
