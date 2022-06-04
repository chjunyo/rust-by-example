fn drink(beverage: &str) {
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }
    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}

/*
Some refreshing water is all I need.
thread 'main' panicked at 'AAAaaaaa!!!!', .\1-panic.rs:2:33
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/