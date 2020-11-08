struct Noisy {
  id: i32,
}

// method called when a instance of the data type goes out of the scope
impl Drop for Noisy {
  fn drop(&mut self) {
      println!("Noisy number {} going out of scope!", self.id);
  }
}

fn main() {
  let _n1 = Noisy { id: 1 };
  let _n2 = Noisy { id: 2 };
  println!("End of main");
}