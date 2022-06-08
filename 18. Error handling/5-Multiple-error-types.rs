fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    println!("The first doubled is {}", double_first(empty));
    println!("The first doubled is {}", double_first(strings));

    /*
    The first doubled is 84
    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', .\5-Multiple-error-types.rs:2:29
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
}
