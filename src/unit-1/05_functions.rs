fn next_birthday(name: &str, current_age: u8) {
  let next_age = current_age + 1;
  println!("Hi {}, on your next birthday, you'll be {}!", name, next_age);
}

fn square(num: i32) -> i32 {
  num * num
}

fn discount(day_of_month: u8) {
  let amount = if day_of_month % 2 == 0 {
    50
  } else {
    10
  };

  println!("your discount is {}", amount);
}

fn main() {
  next_birthday("Cristian David Ippolito", 23);
  next_birthday("Karen Cortes", 21);
  println!("the answer is {}", square(3));
  discount(10);
}