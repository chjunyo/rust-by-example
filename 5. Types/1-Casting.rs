#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // let integer: u8 = decimal; // Error

    let integer = decimal as u8;
    let character = integer as char;

    // let character = decimal as char; // Error

    println!("Casting: {} -> {} -> {}", decimal, integer, character); // 65.4321 -> 65 -> A

    println!("1000 as a u16 is: {}", 1000 as u16); // 1000
    println!("1000 as a u8  is: {}", 1000 as u8); // 1000 - 256 - 256 - 256 = 232
    println!("  -1 as a u8  is: {}", (-1i8) as u8); //-1 + 256 = 255
    println!("1000 mod 256  is: {}", 1000 % 256); //232

    println!(" 128 as a i16 is: {}", 128 as i16); // 128
    println!(" 128 as a i8  is: {}", 128 as i8); // -128

    println!("1000 as a u8  is: {}", 1000 as u8); // 232
    println!(" 232 as a i8  is: {}", 232 as i8); // -24

    println!("300.0 is {}", 300.0_f32 as u8); // 255
    println!("-100.0 as u8 is {}", -100.0_f32 as u8); // 0
    println!("nan as u8 is {}", f32::NAN as u8); // 0

    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>()); // 44
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>()); // 156
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>()); // 0
    }
}
