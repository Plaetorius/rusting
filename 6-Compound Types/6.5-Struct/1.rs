
// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let _p = Person {
        name: String::from("sunface"),
        age: age,
		hobby: String::from("coding"),
    };

    println!("Success!");
} 