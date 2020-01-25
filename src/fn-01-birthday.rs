fn next_birthday(name: &str, current_age: u8) {
  let next_age = current_age + 1;
  println!("Hi {}, on your next birthday, you'll be {}!", name, next_age);
}

fn main() {
  next_birthday("Cristian David Ippolito", 23);
  next_birthday("Karen Cortes", 21);
}