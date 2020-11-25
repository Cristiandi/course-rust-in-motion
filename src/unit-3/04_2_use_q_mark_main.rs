fn main() -> Result<(), Box<std::error::Error>> {
    let x: i32 = "3".parse()?;

    println!("x: {:?}", x);

    Ok(())
}