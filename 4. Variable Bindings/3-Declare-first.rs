fn main() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding); // 4

    let another_binding;

    // println!("another binding: {}", another_binding); // Error
    
    another_binding = 1;

    println!("another binding: {}", another_binding); // 1
}