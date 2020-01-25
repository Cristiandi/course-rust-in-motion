use std::io;

fn main() {
  let mut word = String::new();

  while word.trim() != "rust" {
    println!("what's the secret word?");
    word = String::new();
    io::stdin().read_line(&mut word).expect("falied to read line");
  }

  println!("well donde! You have the secret word.");
}