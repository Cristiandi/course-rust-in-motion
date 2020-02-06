use std::io;

fn main() {
  // loop demo
  loop {
    println!("what's the secret word?");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("falied to read line");

    if word.trim() == "rust" {
      break;
    }
  }

  println!("well donde! You have the secret word.");

  // while demo
  let mut word = String::new();

  while word.trim() != "rust" {
    println!("what's the secret word?");
    word = String::new();
    io::stdin().read_line(&mut word).expect("falied to read line");
  }

  println!("well donde! You have the secret word.");

  // for demo
  for i in 1..11 {
    println!("nor iterating the number {}", i);
  }

  // match demo
  let die1 = 1;
  let die2 = 5;

  match (die1, die2) {
    (1, 1) => println!("snake eyes, go back to the be beginning."),
    (5, _) | (_, 5) => {
      println!("you rolled al least one 5!");
      println!("move and then roll againg!")
    },
    _ => println!("move you piece!")
  }

  // match demo 2
  let is_confirmed = true;
  let is_active = false;

  match (is_confirmed, is_active) {
    (true, true) => println!("this account is in good standing."),
    (false, true) => println!("need to confirm your account!"),
    (false, false) => println!("this account will be desactivated."),
    _ => println!("no controlled")
  }
}