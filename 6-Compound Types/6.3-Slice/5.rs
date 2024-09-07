fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[..3]; // '你' is 3 bytes

    assert!(slice == "你");

    println!("Success!");
}