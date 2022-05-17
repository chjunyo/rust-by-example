fn main() {
    fn function(i: i32) -> i32 {
        i + i
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i)); // function: 2
    println!("clousre_annotated: {}", closure_annotated(i)); // clousre_annotated: 2
    println!("closure_inferred: {}", closure_inferred(i)); // closure_inferred: 2

    let one = || 1;
    println!("closure returning one: {}", one()); // closure returning one: 1
}
