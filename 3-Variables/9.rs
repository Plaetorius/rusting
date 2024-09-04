fn main() {
    let (x, y);
    (x,..) = (3, 4); // x takes value 3, rest is ignored
    [.., y] = [1, 2]; // y takes value 2, rest is ignored
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);

    println!("Success!");
} 