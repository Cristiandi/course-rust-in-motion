fn main() {
  let s = String::from("book");

  let pl = pluralize(s.clone());

  println!(
    "I have one {}, you have two {}",
    s,
    pl // clonning the ownership of s variable to be used pluralize fn
  );
}

fn pluralize(mut singular: String) -> String {
  singular.push_str("s");
  singular
}