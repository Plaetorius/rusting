
fn main() {
	let t = (String::from("hello"), String::from("world"));
 
	let _s = t.0; // Only moves the first element of t
 
	// Modify this line only, don't use `_s`
	println!("{:?}", t.1); // So we can access the second element of t
 }