fn main() {
  let s = String::from("book");

  println!(
    "I have one {}, you have two {}",
    s,
    pluralize(s.clone()) // clonning the ownership of s variable to be used pluralize fn
  )
}

fn pluralize(mut s: String) -> String {
  s.push_str("s");
  s
}