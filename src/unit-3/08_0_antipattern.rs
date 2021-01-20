// Result: is_ok, is_error
// Option: is_some, is_none

// esto es un anti patron porque primero tengo que leer detalladamente
// para entender que el codigo no entrara en panico y ademas se esta
// haciendo un checkeo redundante

fn anti_pattern () {
    let option_value = Some(25);

    if option_value.is_some() {
        let inner = option_value.unwrap();
        println!("inner = {}", inner);
    }
}

fn better_choice() {
    let option_value = Some(25);
    
    if let Some(inner) = option_value {
        println!("inner = {}", inner);
    }
}

fn main () {
    anti_pattern();
}