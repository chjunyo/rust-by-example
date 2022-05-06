fn main() {
    let elem = 5u8;

    let mut vec = Vec::new(); // vector: a growable array, Vec<_>
    vec.push(elem); // Vec<u8>

    println!("{:?}", vec);
}
