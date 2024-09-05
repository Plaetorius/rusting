
fn main() {
	let t = (String::from("hello"), String::from("world"));

		// Fill the blanks
		let (s1, s2): (String, String) = (t.0.clone(), t.1.clone());


		println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }