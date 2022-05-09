#![allow(unreachable_code)]

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }

    /*
        Let's count until infinity!
        1
        2
        three
        4
        5
        OK, that's enough
    */

    // Nesting and labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    /*
        Entered the outer loop
        Entered the inner loop
        Exited the outer loop
    */

    // Returning from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
