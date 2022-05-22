fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");
    call_me(closure); // I'm a closure!
    call_me(function); // I'm a function!
}
