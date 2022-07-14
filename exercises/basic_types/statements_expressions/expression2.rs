// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2; // x += 2 -- this is incrementing the value not return value so () is returned
        x
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }

//  fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2 // x += 2 -- this is incrementing the value not return value so () is returned
//     };
 
//     assert_eq!(v, ()); //assert_eq!(v, 3); -- v has value ()
 
//     println!("Success!");
//  }