extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String
}

fn main () {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margaret Hamilton"
    }"#);

    // se especifica el fallback value
    let first_inner = first.unwrap_or(Person { name: String::from("unknow") });

    println!("first's name = {:?}", first_inner.name);
}