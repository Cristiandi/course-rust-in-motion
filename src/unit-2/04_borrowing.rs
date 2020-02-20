fn main() {
  let s = String::from("book");

  let pl = pluralize(&s);

  println!(
    "I have one {}, you have two {}",
    s,
    pl // clonning the ownership of s variable to be used pluralize fn
  );
}

fn pluralize(singular: &str) -> String {
  singular.to_owned() + "s"
}