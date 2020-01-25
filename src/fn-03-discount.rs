fn discount(day_of_month: u8) {
  let amount = if day_of_month % 2 == 0 {
    50
  } else {
    10
  };

  println!("your discount is {}", amount);
}

fn main() {
  discount(10);
}