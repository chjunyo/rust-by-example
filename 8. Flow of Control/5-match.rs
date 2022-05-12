#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
}

fn main() {
    let number = 13;

    println!("Tell me about {}", number); // Tell me about 13
    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"), // A teen
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary); // true -> 1

    // Destructuring
    // tuples
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple); // Tell me about (0, -2, 3)
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z), // First is `0`, `y` is -2, and `z` is 3
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    // arrays/slices
    let array = [2, -2, 6];
    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}", // array[0] = 2, middle = [-2], array[2] = 6
            first, middle, last
        ),
    }

    // enums
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b), // Red: 122, green: 17, and blue: 40!
    }

    // pointers/ref
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val), // Got a value via destructuring: 4
    }
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val), // Got a value via destructuring: 4
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r), // Got a reference to a value: 5
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m); // We added 10. `mut_value`: 16
        }
    }

    // structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y), // First of x is 1, b = 2, y = 3
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}
