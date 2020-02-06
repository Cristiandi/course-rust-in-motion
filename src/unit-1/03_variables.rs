fn main() {
  let a = 5;
  println!("Hi I'm a no mutable var {}", a);
  let mut b = 6;
  println!("Hi I'm a mutable var {}", b);
  b += 1;
  println!("Yes I changed my value {}", b);
}