fn main() {
    // Fill the blank
    // let list: [i32; 100] = [1; 100]; // Array of 100 1
	let list: [i32; 100] = (1..101).collect();

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}