// Fix errors
fn main() {
    let mut s: String = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter = first_letter(&s);

	
    println!("the first letter is: {}", letter);
    s.clear(); // let the memory go after we are done using it
}
fn first_letter(s: &str) -> &str {
    &s[..1]
}