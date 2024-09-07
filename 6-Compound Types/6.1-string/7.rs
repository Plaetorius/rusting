
// Fix error with at least two solutions
// Sol 1
fn main() {
    let s: &str = "hello, world";
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}", s)
}

// Sol 2
// fn main() {
//     let s = String::from("hello, world");
//     greetings(s)
// }

// fn greetings(s: String) {
//     println!("{}", s)
// }