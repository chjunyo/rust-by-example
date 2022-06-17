fn main() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains {}", immutable_box); // immutable_box contains 5

    //*immutable_box = 4; // Error

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box); // mutable_box contains 5

    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box); // mutable_box now contains 4
}
