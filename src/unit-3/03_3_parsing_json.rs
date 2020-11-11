extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String
}

fn main() {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Cristian David Ippolito"
    }"#);

    let first_inner = match first {
        Ok(inner) => inner,
        Err(error) => {
            if error.is_eof() {
                panic!("sorry we can't handle the is_oef.");
            } else {
                Person {
                    name: String::from("un")
                }
            }
        }
    };

    println!("first = {:?}", first_inner.name);

    let second = serde_json::from_str::<Person>(r#"{
        "name": "Cristian David Ippolito",
    }"#);

    println!("second = {:?}", second);
}