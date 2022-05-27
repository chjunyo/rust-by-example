#![allow(unused)]
// #![feature(never_type)] // `#![feature]` may not be used on the stable release channel

fn foo() -> ! {
    panic!("This call never returns.");
}

fn some_fn() {
    ()
}

fn main() {
    let a: () = some_fn();
    println!("This function returns and you can see this line."); // This function returns and you can see this line.

    // let x: ! = panic!("This call never returns.");
    // println!("You will never see this line!");

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9)); // Sum of odd numbers up to 9 (excluding): 16
}