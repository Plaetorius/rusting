
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // Do some stuff
        }
        _ => {
            // Do some stuff
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
}

// fn never_return_fn() -> ! {
//     uninplemented!()
// }

// fn never_return_fn() -> ! {
//     todo!()
// }