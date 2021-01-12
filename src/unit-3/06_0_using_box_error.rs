use std::env;
use std::error::Error;

// Box<Error> es un trait object
// y es la solución mas sencilla a una función que pueda resultar en multiples tipos de errores
fn num_threads() -> Result<usize, Box<Error>> {
    let s = env::var("NUM_THREADS")?;
    let n: usize = s.parse()?;

    Ok(n + 1)
}

// Lo que este tipo significa es que esta funcion va a retornar algun tipo de error que implemente Error
// pero el programa no sabe el tipo concreto del error hasta el runtime
fn run_application() -> Result<(), Box<Error>> {
    let num = num_threads()?;

    println!("The number of threads is {}", num);

    // Rest of program functionality

    Ok(())
}

fn main() {
    if let Err(e) = run_application() {
        panic!("An error happened: {}", e);
    }
}