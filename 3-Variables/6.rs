
// Remove a line in the code to make it compile
fn main() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    _x += 3;


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!"; 

    println!("Success!");
}