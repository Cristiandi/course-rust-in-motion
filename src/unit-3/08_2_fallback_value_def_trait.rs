extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Deserialize, Debug)]
struct Person {
    name: String
}

// implementando el default trait
impl Default for Person {
    fn default() -> Self {
        Person {
            name: String::from("unknown")
        }
    }
}

fn main () {
    let first = serde_json::from_str::<Person>(r#"{
        "name": "Margaret Hamilton"
    }"#);

    // no se especifica el default alue porque ya se implemento el Default trait
    let first_inner = first.unwrap_or_default();

    println!("first's name = {:?}", first_inner.name);
}