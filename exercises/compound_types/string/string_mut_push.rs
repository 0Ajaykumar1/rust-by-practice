// Fix all errors without adding newline
fn main() {
    let mut s =  String::from("hello"); //let  s =  String::from("hello");
    s.push(',');
    s.push_str(" world"); //s.push(" world");
    s += "!"; //s += "!".to_string();

    println!("{}", s);
}