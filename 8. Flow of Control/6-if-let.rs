enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i); // This is a really long string and `7`
        }
        _ => {}
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i); // Matched 7!
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!"); // I don't like letters. Let's go with an emoticon :)!
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar"); // a is foobar
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value); // c is 100
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred"); // c is one hundred
    }

    if let Foo::Bar = a {
        println!("a is foobar"); // a is foobar
    }
}
