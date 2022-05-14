fn main() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }

    /*
    `i` is `0`. Try again.
    `i` is `1`. Try again.
    `i` is `2`. Try again.
    `i` is `3`. Try again.
    `i` is `4`. Try again.
    `i` is `5`. Try again.
    `i` is `6`. Try again.
    `i` is `7`. Try again.
    `i` is `8`. Try again.
    `i` is `9`. Try again.
    Greater than 9, quit!
    */

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }

    /*
    `i` is `0`. Try again.
    `i` is `1`. Try again.
    `i` is `2`. Try again.
    `i` is `3`. Try again.
    `i` is `4`. Try again.
    `i` is `5`. Try again.
    `i` is `6`. Try again.
    `i` is `7`. Try again.
    `i` is `8`. Try again.
    `i` is `9`. Try again.
    Greater than 9, quit!
    */
}
