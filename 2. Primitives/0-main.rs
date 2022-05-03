fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12;
    inferred_type = 4294967296i64; // i64

    let mut mutable = 12; // Mutable(mut) i32
    mutable = 21;

    //mutable = true; // error

    let mutable = true; // Variables can be overwritten with shadowing.
}