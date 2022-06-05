fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);

    /*
    water? How nice.
    Yuck! Too sugary.
    No drink? Oh well.
    I love coffees!!!!!
    thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', .\3-Option-unwrap.rs:10:24
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
}