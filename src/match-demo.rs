fn main() {
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
}