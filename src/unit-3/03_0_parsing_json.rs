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

    println!("first = {:?}", first);

    let second = serde_json::from_str::<Person>(r#"{
        "name": "Cristian David Ippolito",
    }"#);

    println!("second = {:?}", second);
}