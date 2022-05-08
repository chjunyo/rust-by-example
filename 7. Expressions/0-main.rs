fn main() {
    let x = 5;

    x;
    x + 11;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x); // 5
    println!("y is {:?}", y); // 155
    println!("z is {:?}", z); // ()
}
