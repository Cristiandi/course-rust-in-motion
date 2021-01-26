// Concrete lifetime of a value
// En Rust el value lifetime es el tiempo durante el cual el valor existe en una dirección particular de la memoria
// se puede pensar en la validez de la direccion de memoria como la validez de la dirección de la casa de una persona,
// cuando se nace se vive en una dirección particular, en algun punto, uno se cambia de casa, si alguien tiene la
// dirección vieja y trata de visitar, no estara ahí.

fn main() {
    let list = vec![100, 34, 72, 55];

    let first_two = &list[0..2];

    println!("first two are {:?}", first_two);

    println!("list is {:?}", list);

    println!("again, first two are {:?}", first_two);
}