// Diverging functions never return to the caller, 
// so they may be used in places where a value of any type is expected.


fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!()
}

// Another way
// fn never_return_fn() -> ! {
//     panic!()
// }

// one more way is 
// fn never_return_fn() -> ! {
//     loop {
//         std::thread::sleep(std::time::Duration::from_secs(1))
//     }
// }
