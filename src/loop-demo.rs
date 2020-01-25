use std::io;

fn main() {
  loop {
    println!("what's the secret word?");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("falied to read line");

    if word.trim() == "rust" {
      break;
    }
  }

  println!("well donde! You have the secret word.");
}