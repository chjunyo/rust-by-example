use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        // Try 'write!' to see if it errors. If it errors, return the error. Otherwise continue.
        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?; // Try Activity
        }
        write!(f, "]") // Close the opened bracket and return a fmt::Result value.
    }
}

fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:0>width$}", number=1, width=6); // output: 000001

    println!("Now {:?} will print!", Structure(3));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
    /*
        Person {
            name: "Peter",
            age: 27,
        }
    */

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Display: {}", point); // Display: x: 3.3, y: 7.2
    println!("Debug: {:?}", point); // Debug: Point2D { x: 3.3, y: 7.2 }

    let v = List(vec![1, 2, 3]);
    println!("{}", v); // [0: 1, 1: 2, 2: 3]
}