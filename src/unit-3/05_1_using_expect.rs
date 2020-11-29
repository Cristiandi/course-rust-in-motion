fn main () {
    let num_a: i32 = "10".parse()
                    .expect("Parsing failed.");

    println!("num_a: {}", num_a);

    let num_b: i32 = "apple".parse()
                        .expect("Parsion failed.");

    println!("num_b: {}", num_b);
}