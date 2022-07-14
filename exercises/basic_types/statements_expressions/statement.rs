fn main() {
    let v = {let x = 3; x}; //let v = (let x = 3); -- block statement should return the value instead of assignment
 
    assert!(v == 3);
 
    println!("Success!");
 }