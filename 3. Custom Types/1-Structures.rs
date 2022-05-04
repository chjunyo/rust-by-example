#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rect;
    let Point { x: x1, y: y1 } = top_left;
    let Point { x: x2, y: y2 } = bottom_right;
    (x2 - x1) * (y2 - y1)
}

fn square(point: Point, w: f32) -> Rectangle {
    let Point { x, y } = point;
    Rectangle { top_left: point, bottom_right: Point { x: x + w, y: y + w } }
}

fn main() {
    let name = String::from("Peter"); // String::from: heap allocated, "": no heap memory
    let age =  27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y); // (5.2, 0.4);

    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    //Activity
    println!("rect_area: {:?}", rect_area(square(Point { x: 5.0, y: 5.0 }, 0.5)));
    let Rectangle { top_left: tl, bottom_right: br } = square(point, 0.5);
    println!("square: ({}, {}), ({}, {})", tl.x, tl.y, br.x, br.y);
}